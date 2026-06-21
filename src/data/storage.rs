use super::stats::Stats;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Write `content` atomically: write to a sibling temp file, then rename over the
/// target. Rename is atomic on the same filesystem, so an interrupted write cannot
/// truncate or corrupt the existing file.
fn atomic_write(path: &Path, content: &str) -> io::Result<()> {
    let tmp_path = path.with_extension("tmp");
    fs::write(&tmp_path, content).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to write temp file {}: {}", tmp_path.display(), e),
        )
    })?;
    fs::rename(&tmp_path, path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!(
                "Failed to replace {} with {}: {}",
                path.display(),
                tmp_path.display(),
                e
            ),
        )
    })
}

/// Move a corrupt JSON file aside to `<path>.bak` so the app can start fresh instead of
/// failing to launch. Best-effort: a failed backup is reported but not fatal.
fn back_up_corrupt_file(path: &Path) {
    let backup = path.with_extension("bak");
    if let Err(e) = fs::rename(path, &backup) {
        eprintln!(
            "Warning: could not back up corrupt file {} to {}: {}",
            path.display(),
            backup.display(),
            e
        );
    } else {
        eprintln!(
            "Warning: {} was corrupt and has been moved to {}; starting fresh.",
            path.display(),
            backup.display()
        );
    }
}

/// Gestionnaire de stockage des stats
pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> io::Result<Self> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir)?;

        let file_path = config_dir.join("stats.json");

        Ok(Self { file_path })
    }

    /// Create a Storage instance with a custom file path (used for testing)
    #[cfg(test)]
    fn with_path(file_path: PathBuf) -> io::Result<Self> {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(Self { file_path })
    }

    /// Récupérer le dossier de configuration
    fn get_config_dir() -> io::Result<PathBuf> {
        let home = std::env::var("HOME").map_err(|_| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Cannot locate config directory: HOME is not set. Set HOME to your home directory.",
            )
        })?;

        Ok(PathBuf::from(home).join(".config").join("typer-cli"))
    }

    /// Charger les stats depuis le fichier.
    /// A corrupt file is moved aside and defaults are returned, so the app always launches.
    pub fn load(&self) -> io::Result<Stats> {
        if !self.file_path.exists() {
            return Ok(Stats::new());
        }

        let content = fs::read_to_string(&self.file_path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to read stats {}: {}", self.file_path.display(), e),
            )
        })?;

        match serde_json::from_str(&content) {
            Ok(stats) => Ok(stats),
            Err(_) => {
                back_up_corrupt_file(&self.file_path);
                Ok(Stats::new())
            }
        }
    }

    /// Sauvegarder les stats dans le fichier (atomic write).
    pub fn save(&self, stats: &Stats) -> io::Result<()> {
        let content = serde_json::to_string_pretty(stats).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Failed to serialize stats for {}: {}",
                    self.file_path.display(),
                    e
                ),
            )
        })?;

        atomic_write(&self.file_path, &content)
    }

    /// A corrupt config file is moved aside and defaults are returned, so the app always launches.
    pub fn load_config(&self) -> io::Result<crate::data::Config> {
        let config_path = self.config_path();
        if !config_path.exists() {
            return Ok(crate::data::Config::default());
        }
        let content = fs::read_to_string(&config_path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to read config {}: {}", config_path.display(), e),
            )
        })?;
        match serde_json::from_str(&content) {
            Ok(config) => Ok(config),
            Err(_) => {
                back_up_corrupt_file(&config_path);
                Ok(crate::data::Config::default())
            }
        }
    }

    pub fn save_config(&self, config: &crate::data::Config) -> io::Result<()> {
        let config_path = self.config_path();
        let content = serde_json::to_string_pretty(config).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Failed to serialize config for {}: {}",
                    config_path.display(),
                    e
                ),
            )
        })?;
        atomic_write(&config_path, &content)
    }

    fn config_path(&self) -> PathBuf {
        // Fall back to the current directory rather than panicking if the stats path
        // somehow has no parent.
        self.file_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("config.json")
    }

    /// Public API: Path accessor for debugging, data export/migration, and future admin features
    pub fn get_path(&self) -> &PathBuf {
        &self.file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::stats::SessionRecord;
    use std::time::Duration;

    /// Helper to create a temporary test storage path
    fn create_test_storage() -> (Storage, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_stats.json");
        let storage = Storage::with_path(file_path).unwrap();
        (storage, temp_dir)
    }

    #[test]
    fn test_storage_new() {
        let storage = Storage::new();
        assert!(storage.is_ok());
    }

    #[test]
    fn test_load_empty_stats() {
        let (storage, _temp_dir) = create_test_storage();
        // When stats file doesn't exist, load should return empty stats
        let stats = storage.load();
        assert!(stats.is_ok());
        assert_eq!(stats.unwrap().session_count(), 0);
    }

    #[test]
    fn test_save_and_load() {
        let (storage, _temp_dir) = create_test_storage();

        let mut stats = Stats::new();
        stats.add_session(SessionRecord::new(
            "HomeRow-1".to_string(),
            45.0,
            95.0,
            Duration::from_secs(60),
            Duration::from_secs(300),
        ));

        // Sauvegarder
        let save_result = storage.save(&stats);
        assert!(save_result.is_ok());

        // Charger
        let loaded_stats = storage.load().unwrap();
        assert_eq!(loaded_stats.session_count(), 1);
    }

    #[test]
    fn test_load_config_defaults_when_missing() {
        let (storage, _temp_dir) = create_test_storage();
        let config = storage.load_config().unwrap();
        assert_eq!(config.layout_variant, crate::keyboard::LayoutVariant::Mac);
    }

    #[test]
    fn test_save_and_load_config() {
        let (storage, _temp_dir) = create_test_storage();
        let config = crate::data::Config {
            layout_variant: crate::keyboard::LayoutVariant::Pc,
        };
        storage.save_config(&config).unwrap();
        let loaded = storage.load_config().unwrap();
        assert_eq!(loaded.layout_variant, crate::keyboard::LayoutVariant::Pc);
    }

    #[test]
    fn test_load_config_corrupt_json_recovers_to_default() {
        // A corrupt config must not block launch: it is backed up and defaults returned.
        let (storage, temp_dir) = create_test_storage();
        let config_path = temp_dir.path().join("config.json");
        fs::write(&config_path, b"not valid json").unwrap();

        let config = storage.load_config().unwrap();
        assert_eq!(config.layout_variant, crate::keyboard::LayoutVariant::Mac);
        // Corrupt file moved aside.
        assert!(!config_path.exists());
        assert!(temp_dir.path().join("config.bak").exists());
    }

    #[test]
    fn test_load_corrupt_stats_recovers_to_empty() {
        // A corrupt stats file must not block launch.
        let (storage, temp_dir) = create_test_storage();
        fs::write(temp_dir.path().join("test_stats.json"), b"{ broken json").unwrap();

        let stats = storage.load().unwrap();
        assert_eq!(stats.session_count(), 0);
        assert!(!temp_dir.path().join("test_stats.json").exists());
        assert!(temp_dir.path().join("test_stats.bak").exists());
    }

    #[test]
    fn test_save_leaves_no_temp_file() {
        // Atomic write must rename the temp file away, not leave it behind.
        let (storage, temp_dir) = create_test_storage();
        storage.save(&Stats::new()).unwrap();

        assert!(temp_dir.path().join("test_stats.json").exists());
        assert!(!temp_dir.path().join("test_stats.tmp").exists());
    }

    #[test]
    fn test_save_preserves_previous_file_on_serialize_path() {
        // Round-trip through atomic write keeps data intact.
        let (storage, _temp_dir) = create_test_storage();
        let mut stats = Stats::new();
        stats.add_session(SessionRecord::new(
            "HomeRow-1".to_string(),
            45.0,
            95.0,
            Duration::from_secs(60),
            Duration::from_secs(300),
        ));
        storage.save(&stats).unwrap();
        storage.save(&stats).unwrap(); // overwrite existing file atomically

        assert_eq!(storage.load().unwrap().session_count(), 1);
    }
}

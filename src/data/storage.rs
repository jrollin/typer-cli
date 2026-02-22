use super::stats::Stats;
use std::fs;
use std::io;
use std::path::PathBuf;

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
            io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not set")
        })?;

        Ok(PathBuf::from(home).join(".config").join("typer-cli"))
    }

    /// Charger les stats depuis le fichier
    pub fn load(&self) -> io::Result<Stats> {
        if !self.file_path.exists() {
            return Ok(Stats::new());
        }

        let content = fs::read_to_string(&self.file_path)?;
        let stats: Stats = serde_json::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse stats: {}", e),
            )
        })?;

        Ok(stats)
    }

    /// Sauvegarder les stats dans le fichier
    pub fn save(&self, stats: &Stats) -> io::Result<()> {
        let content = serde_json::to_string_pretty(stats).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize stats: {}", e),
            )
        })?;

        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn load_config(&self) -> io::Result<crate::data::Config> {
        let config_path = self.config_path();
        if !config_path.exists() {
            return Ok(crate::data::Config::default());
        }
        let content = fs::read_to_string(&config_path)?;
        serde_json::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse config: {}", e),
            )
        })
    }

    pub fn save_config(&self, config: &crate::data::Config) -> io::Result<()> {
        let content = serde_json::to_string_pretty(config).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize config: {}", e),
            )
        })?;
        fs::write(self.config_path(), content)?;
        Ok(())
    }

    fn config_path(&self) -> std::path::PathBuf {
        self.file_path
            .parent()
            .expect("stats path always has a parent dir")
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
    fn test_load_config_invalid_json_returns_error() {
        let (storage, temp_dir) = create_test_storage();
        let config_path = temp_dir.path().join("config.json");
        fs::write(&config_path, b"not valid json").unwrap();
        let result = storage.load_config();
        assert!(result.is_err());
    }
}

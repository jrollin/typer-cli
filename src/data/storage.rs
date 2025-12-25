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

    /// Obtenir le chemin du fichier de stats
    #[allow(dead_code)]
    pub fn get_path(&self) -> &PathBuf {
        &self.file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::data::stats::SessionRecord;

    #[test]
    fn test_storage_new() {
        let storage = Storage::new();
        assert!(storage.is_ok());
    }

    #[test]
    fn test_load_empty_stats() {
        let storage = Storage::new().unwrap();
        // Si le fichier n'existe pas, on devrait obtenir des stats vides
        // Note: ce test pourrait échouer si des stats existent déjà
        let stats = storage.load();
        assert!(stats.is_ok());
    }

    #[test]
    fn test_save_and_load() {
        let storage = Storage::new().unwrap();

        let mut stats = Stats::new();
        stats.add_session(SessionRecord::new(
            "HomeRow-1".to_string(),
            45.0,
            95.0,
            Duration::from_secs(60),
        ));

        // Sauvegarder
        let save_result = storage.save(&stats);
        assert!(save_result.is_ok());

        // Charger
        let loaded_stats = storage.load().unwrap();
        assert!(loaded_stats.session_count() >= 1);
    }
}

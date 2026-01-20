use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Settings {
    pub steam_id: Option<String>,
}

impl Settings {
    /// Get the path to the settings file in the AppData directory
    fn get_settings_path() -> Option<PathBuf> {
        dirs::data_dir().map(|mut path| {
            path.push("dota-keeper");
            path.push("settings.json");
            path
        })
    }

    /// Load settings from the JSON file, or return default if not found
    pub fn load() -> Self {
        let Some(path) = Self::get_settings_path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save settings to the JSON file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::get_settings_path()
            .ok_or_else(|| "Could not determine settings directory".to_string())?;

        // Create the directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create settings directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&path, json).map_err(|e| format!("Failed to write settings file: {}", e))?;

        Ok(())
    }

    /// Check if this is the first run (no steam_id set)
    pub fn is_first_run(&self) -> bool {
        self.steam_id.is_none()
    }
}

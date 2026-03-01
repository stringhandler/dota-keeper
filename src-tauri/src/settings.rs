use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use uuid;

static SETTINGS_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn set_settings_dir(dir: PathBuf) {
    let _ = SETTINGS_DIR.set(dir);
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AnalyticsConsent {
    Accepted,
    Declined,
    NotYet,
}

impl Default for AnalyticsConsent {
    fn default() -> Self {
        AnalyticsConsent::NotYet
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub steam_id: Option<String>,
    #[serde(default = "Settings::default_difficulty")]
    pub suggestion_difficulty: String,
    #[serde(default)]
    pub suggestion_custom_percentage: Option<f64>,
    #[serde(default)]
    pub analytics_consent: AnalyticsConsent,
    /// Unique installation ID for analytics tracking (generated once, persisted forever)
    #[serde(default = "Settings::generate_installation_id")]
    pub installation_id: String,
    /// Whether mood check-in tracking is enabled (opt-in, default off)
    #[serde(default)]
    pub mental_health_tracking_enabled: bool,
    /// Whether the first-enable explanation modal has been shown
    #[serde(default)]
    pub mental_health_intro_shown: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            steam_id: None,
            suggestion_difficulty: Self::default_difficulty(),
            suggestion_custom_percentage: None,
            analytics_consent: AnalyticsConsent::default(),
            installation_id: Self::generate_installation_id(),
            mental_health_tracking_enabled: false,
            mental_health_intro_shown: false,
        }
    }
}

impl Settings {
    fn default_difficulty() -> String {
        "Medium".to_string()
    }

    /// Generate a unique installation ID (UUID v4)
    fn generate_installation_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Returns the improvement factor range (min, max) based on difficulty
    pub fn improvement_range(&self) -> (f64, f64) {
        match self.suggestion_difficulty.as_str() {
            "Easy" => (0.03, 0.05),
            "Hard" => (0.10, 0.15),
            "Custom" => {
                let pct = self.suggestion_custom_percentage.unwrap_or(0.10);
                (pct - 0.02, pct + 0.02)
            }
            _ => (0.05, 0.10), // Medium
        }
    }

    /// Get the path to the settings file in the AppData directory
    fn get_settings_path() -> Option<PathBuf> {
        let base = match SETTINGS_DIR.get() {
            Some(dir) => dir.clone(),
            None => {
                let mut p = dirs::data_dir()?;
                p.push("dota-keeper");
                p
            }
        };
        Some(base.join("settings.json"))
    }

    /// Load settings from the JSON file, or return default if not found.
    /// If `installation_id` is missing from the file, generates one and saves it immediately
    /// so the same ID is used on all subsequent loads.
    pub fn load() -> Self {
        let Some(path) = Self::get_settings_path() else {
            return Self::default();
        };

        if !path.exists() {
            let settings = Self::default();
            let _ = settings.save();
            return settings;
        }

        match fs::read_to_string(&path) {
            Ok(contents) => {
                // Check whether installation_id was absent before serde fills it in with a fresh UUID.
                let missing_installation_id = serde_json::from_str::<serde_json::Value>(&contents)
                    .map(|v| v.get("installation_id").is_none())
                    .unwrap_or(false);

                let settings: Self = serde_json::from_str(&contents).unwrap_or_default();

                if missing_installation_id {
                    let _ = settings.save();
                }

                settings
            }
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

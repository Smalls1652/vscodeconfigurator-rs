pub mod csharp;

use std::{fs, path::PathBuf};

use serde_json::Value;

/// Represents the settings file for a Visual Studio Code workspace.
/// 
/// ## Fields
/// 
/// * `file_path` - The path to the settings file.
/// * `values` - The values in the settings file.
pub struct VSCodeSettings {
    pub file_path: PathBuf,
    pub values: Value
}

impl VSCodeSettings {
    /// Creates a new `VSCodeSettings` instance.
    /// 
    /// ## Arguments
    /// 
    /// * `file_path` - The path to the settings file.
    pub fn new(
        file_path: PathBuf
    ) -> Self {
        if !file_path.exists() {
            panic!("The settings file does not exist.");
        }

        let settings_json = fs::read_to_string(&file_path).unwrap();
        let settings = serde_json::from_str(&settings_json.as_str()).unwrap();

        Self {
            file_path: file_path,
            values: settings
        }
    }

    /// Writes the settings to the settings file.
    pub fn write_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let updated_settings_json = serde_json::to_string_pretty(&self.values)?;

        fs::write(&self.file_path, updated_settings_json)?;

        Ok(())
    }
}

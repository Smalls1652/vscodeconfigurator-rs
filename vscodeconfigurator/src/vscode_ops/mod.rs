pub mod csharp;
pub mod rust;

use std::{fs, path::PathBuf};

use serde_json::Value;

/// Represents the settings file for a Visual Studio Code workspace.
/// 
/// ## Fields
/// 
/// * `file_path` - The path to the settings file.
/// * `values` - The values in the settings file.
pub struct VSCodeSettingsFile {
    pub file_path: PathBuf,
    pub values: Value
}

impl VSCodeSettingsFile {
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

/// Represents the tasks file for a Visual Studio Code workspace.
pub struct VSCodeTasksFile {
    /// The path to the tasks file.
    pub file_path: PathBuf,

    /// The values in the tasks file.
    pub values: Value
}

impl VSCodeTasksFile {
    /// Creates a new `VSCodeTasks` instance.
    /// 
    /// ## Arguments
    /// 
    /// * `file_path` - The path to the tasks file.
    pub fn new(
        file_path: PathBuf
    ) -> Self {
        if !file_path.exists() {
            panic!("The tasks file does not exist.");
        }

        let tasks_json = fs::read_to_string(&file_path).unwrap();
        let tasks = serde_json::from_str(&tasks_json.as_str()).unwrap();

        Self {
            file_path: file_path,
            values: tasks
        }
    }

    /// Writes the tasks to the tasks file.
    pub fn write_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let updated_tasks_json = serde_json::to_string_pretty(&self.values)?;

        fs::write(&self.file_path, updated_tasks_json)?;

        Ok(())
    }
}

use std::path::PathBuf;

use crate::console_utils::{ConsoleUtils, OutputEmoji};

/// Creates the `.vscode` directory in the project root if it does not exist.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `console_utils` - The [`ConsoleUtils`](crate::console_utils::ConsoleUtils)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Create the `.vscode` directory in the project root if it does not exist.
///
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let mut console_utils = ConsoleUtils::new();
///
/// ensure_vscode_dir_exists(&output_directory, console_utils);
/// ```
pub fn ensure_vscode_dir_exists(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_dir_path = output_directory.join(".vscode");

    if !vscode_dir_path.exists() {
        console_utils
            .write_operation_log("Creating '.vscode' directory...", OutputEmoji::Folder)?;
        std::fs::create_dir(&vscode_dir_path)?;
        console_utils.write_operation_success_log()?;
    }

    Ok(())
}

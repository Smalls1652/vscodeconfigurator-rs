use std::path::PathBuf;

use vscodeconfigurator_lib::logging::{ConsoleLogger, OutputEmoji};

/// Creates the `.vscode` directory in the project root if it does not exist.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `logger` - The [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger)
///   instance for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Create the `.vscode` directory in the project root if it does not exist.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let mut logger = ConsoleLogger::new();
///
/// ensure_vscode_dir_exists(&output_directory, logger);
/// ```
pub fn ensure_vscode_dir_exists(
    output_directory: &PathBuf,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_dir_path = output_directory.join(".vscode");

    if !vscode_dir_path.exists() {
        logger
            .write_operation_log("Creating '.vscode' directory...", OutputEmoji::Folder)?;
        std::fs::create_dir(&vscode_dir_path)?;
        logger.write_operation_success_log()?;
    }

    Ok(())
}

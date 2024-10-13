use std::path::PathBuf;

use crate::logging::{ConsoleLogger, OutputEmoji};

/// Initializes a Git repository in the output directory.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `logger` - The
///   [`ConsoleLogger`](vscodeconfigurator_lib::logging::ConsoleLogger) instance
///   for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Initializes a Git repository in the `my-project` directory in the temp
/// directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let mut logger = ConsoleLogger::new();
///
/// initialize_git_repo(&output_directory, logger);
/// ```
pub fn initialize_git_repo(
    output_directory: &PathBuf,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log("Initializing Git repository...", OutputEmoji::Package)?;

    let git_proc_args = vec!["init"];

    std::process::Command::new("git")
        .args(git_proc_args)
        .current_dir(output_directory)
        .output()?;

    logger.write_operation_success_log()?;
    Ok(())
}

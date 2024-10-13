use std::path::PathBuf;

use crate::console_utils::{ConsoleUtils, OutputEmoji};

/// Initializes a Git repository in the output directory.
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
/// Initializes a Git repository in the `my-project` directory in the temp
/// directory.
///
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let mut console_utils = ConsoleUtils::new();
///
/// initialize_git_repo(&output_directory, console_utils);
/// ```
pub fn initialize_git_repo(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_operation_log("Initializing Git repository...", OutputEmoji::Package)?;

    let git_proc_args = vec!["init"];

    std::process::Command::new("git")
        .args(git_proc_args)
        .current_dir(output_directory)
        .output()?;

    console_utils.write_operation_success_log()?;
    Ok(())
}

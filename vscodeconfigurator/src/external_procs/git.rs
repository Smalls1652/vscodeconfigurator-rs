use std::path::PathBuf;

use crate::console_utils::{ConsoleUtils, OutputEmoji};

/// Initializes a Git repository in the output directory.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `console_utils` - The console utilities.
pub fn initialize_git_repo(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_operation_log("Initializing Git repository...", OutputEmoji::Package)?;
    
    let git_proc_args = vec![
        "init"
    ];

    std::process::Command::new("git")
        .args(git_proc_args)
        .current_dir(output_directory)
        .output()?;

    console_utils.write_operation_success_log()?;
    Ok(())
}

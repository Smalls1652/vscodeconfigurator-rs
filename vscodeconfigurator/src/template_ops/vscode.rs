use std::path::PathBuf;

use crate::console_utils::ConsoleUtils;

/// Copies the `settings.json` file to the project root's `.vscode` directory.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `console_utils` - The console utilities.
pub fn ensure_vscode_dir_exists(output_directory: &PathBuf, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_dir_path = output_directory.join(".vscode");

    if !vscode_dir_path.exists() {
        console_utils.write_info(format!("- 📁 Creating '.vscode' directory... "))?;
        std::fs::create_dir(&vscode_dir_path)?;
        console_utils.write_success(format!("Done! ✅\n"))?;
    }
    
    Ok(())
}

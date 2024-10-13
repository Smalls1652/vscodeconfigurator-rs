use std::{fs, path::PathBuf};

use crate::{console_utils::{ConsoleUtils, OutputEmoji}, utils};

use super::vscode;

/// Copies the `GitVersion.yml` file to the project root.
/// 
/// # Arguments
/// 
/// - `output_directory` - The output directory of the project.
/// - `force` - Whether to forcefully overwrite.
/// - `console_utils` - The [`ConsoleUtils`](crate::console_utils::ConsoleUtils) instance for logging.
/// 
/// # Examples
/// 
/// ## Example 01
/// 
/// Copies the `GitVersion.yml` file to the project root.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///    .join("my-project");
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// csharp_copy_gitversion(&output_directory, force, console_utils);
/// ```
pub fn csharp_copy_gitversion(output_directory: &PathBuf, force: bool, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("csharp/GitVersion/GitVersion.yml");

    let output_file_name = "GitVersion.yml";
    let output_file_path = output_directory.join(&output_file_name);

    console_utils.write_operation_log("Copying 'GitVersion.yml' to project root...", OutputEmoji::Document)?;

    if output_file_path.exists() {
        if !force {
            let overwrite_response = console_utils.ask_for_overwrite()?;

            if !overwrite_response {
                
                console_utils.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(&output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());

        
    }

    fs::copy(template_file_path, &output_file_path)?;

    console_utils.write_operation_success_log()?;

    Ok(())
}

/// Copies the `settings.json` file to the project root's `.vscode` directory.
/// 
/// # Arguments
/// 
/// - `output_directory` - The output directory of the project.
/// - `solution_name` - The name of the solution.
/// - `force` - Whether to forcefully overwrite.
/// - `console_utils` - The [`ConsoleUtils`](crate::console_utils::ConsoleUtils) instance for logging.
/// 
/// # Examples
/// 
/// ## Example 01
/// 
/// Copies the `settings.json` file to the project root's `.vscode` directory.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///   .join("my-project");
/// let solution_name = "MySolution".to_string();
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// csharp_copy_vscode_settings(&output_directory, &solution_name, force, console_utils);
/// ```
pub fn csharp_copy_vscode_settings(
    output_directory: &PathBuf,
    solution_name: &String,
    force: bool,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, console_utils)?;

    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("csharp/VSCode/settings.json");

    let output_file_name = "settings.json";
    let output_file_path = output_directory.join(".vscode").join(&output_file_name);

    console_utils.write_operation_log("Copying 'settings.json' to '.vscode' directory...", OutputEmoji::Document)?;

    if output_file_path.exists() {
        if !force {
            let overwrite_response = console_utils.ask_for_overwrite()?;

            if !overwrite_response {
                
                console_utils.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(&output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());

        
    }

    let solution_name_full = format!("{:}.sln", solution_name);
    let vscode_settings_json = fs::read_to_string(&template_file_path)?
        .replace("{{solutionName}}", &solution_name_full);

    fs::write(&output_file_path, vscode_settings_json)?;

    console_utils.write_operation_success_log()?;

    Ok(())
}

/// Copies the `tasks.json` file to the project root's `.vscode` directory.
/// 
/// # Arguments
/// 
/// - `output_directory` - The output directory of the project.
/// - `solution_name` - The name of the solution.
/// - `force` - Whether to forcefully overwrite.
/// - `console_utils` - The [`ConsoleUtils`](crate::console_utils::ConsoleUtils) instance for logging.
/// 
/// # Examples
/// 
/// ## Example 01
/// 
/// Copies the `tasks.json` file to the project root's `.vscode` directory.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///  .join("my-project");
/// let solution_name = "MySolution".to_string();
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// csharp_copy_vscode_tasks(&output_directory, &solution_name, force, console_utils);
/// ```
pub fn csharp_copy_vscode_tasks(
    output_directory: &PathBuf,
    solution_name: &String,
    force: bool,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, console_utils)?;

    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("csharp/VSCode/tasks.json");

    let output_file_name = "tasks.json";
    let output_file_path = output_directory.join(".vscode").join(&output_file_name);

    console_utils.write_operation_log("Copying 'tasks.json' to '.vscode' directory...", OutputEmoji::Document)?;

    if output_file_path.exists() {
        if !force {
            let overwrite_response = console_utils.ask_for_overwrite()?;

            if !overwrite_response {
                
                console_utils.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(&output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());

        
    }

    let solution_name_full = format!("{:}.sln", solution_name);
    let vscode_tasks_json = fs::read_to_string(&template_file_path)?
        .replace("{{solutionName}}", &solution_name_full);

    fs::write(&output_file_path, vscode_tasks_json)?;

    console_utils.write_operation_success_log()?;

    Ok(())
}

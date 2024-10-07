use std::{fs, path::PathBuf};

use crate::{console_utils::ConsoleUtils, utils};

use super::vscode;

/// Copies the `GitVersion.yml` file to the project root.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `console_utils` - The console utilities.
pub fn csharp_copy_gitversion(output_directory: &PathBuf, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("csharp/GitVersion/GitVersion.yml");

    let output_file_name = "GitVersion.yml";
    let output_file_path = output_directory.join(&output_file_name);

    console_utils.write_info(format!("- 📄 Copying 'GitVersion.yml' to project root... "))?;

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(&output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());

        
    }

    fs::copy(template_file_path, &output_file_path)?;

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Copies the `settings.json` file to the project root's `.vscode` directory.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `console_utils` - The console utilities.
pub fn csharp_copy_vscode_settings(output_directory: &PathBuf, solution_name: &String, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, console_utils)?;

    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("csharp/VSCode/settings.json");

    let output_file_name = "settings.json";
    let output_file_path = output_directory.join(".vscode").join(&output_file_name);

    console_utils.write_info(format!("- 📄 Copying 'settings.json' to '.vscode' directory... "))?;

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(&output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());

        
    }

    let solution_name_full = format!("{:}.sln", solution_name);
    let vscode_settings_json = fs::read_to_string(&template_file_path)?
        .replace("{{solutionName}}", &solution_name_full);

    fs::write(&output_file_path, vscode_settings_json)?;

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Copies the `tasks.json` file to the project root's `.vscode` directory.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `console_utils` - The console utilities.
pub fn csharp_copy_vscode_tasks(output_directory: &PathBuf, solution_name: &String, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, console_utils)?;

    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("csharp/VSCode/tasks.json");

    let output_file_name = "tasks.json";
    let output_file_path = output_directory.join(".vscode").join(&output_file_name);

    console_utils.write_info(format!("- 📄 Copying 'tasks.json' to '.vscode' directory... "))?;

    if output_file_path.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
        }

        fs::remove_file(&output_file_path)
            .expect(format!("Failed to remove existing '{:}' file.", &output_file_name).as_str());

        
    }

    let solution_name_full = format!("{:}.sln", solution_name);
    let vscode_tasks_json = fs::read_to_string(&template_file_path)?
        .replace("{{solutionName}}", &solution_name_full);

    fs::write(&output_file_path, vscode_tasks_json)?;

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

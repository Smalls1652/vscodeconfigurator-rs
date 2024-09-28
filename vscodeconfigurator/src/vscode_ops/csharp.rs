use std::{fs, path::PathBuf};

use serde_json::{json, Value};

use crate::{console_utils::ConsoleUtils, subcommands::csharp::init::CsharpLspOption};

/// Updates the C# LSP option in the `.vscode/settings.json` file.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `csharp_lsp` - The C# language server to use.
/// * `console_utils` - The console utilities.
pub fn update_csharp_lsp(
    output_directory: &PathBuf,
    csharp_lsp: CsharpLspOption,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_settings_path = output_directory.join(".vscode/settings.json");

    console_utils.write_info(format!("- 📄 Updating C# LSP option in tasks.json... "))?;

    let vscode_settings_json = fs::read_to_string(&vscode_settings_path)?;

    let mut vscode_settings: Value = serde_json::from_str(&vscode_settings_json.as_str())?;

    vscode_settings["dotnet.server.useOmnisharp"] = match csharp_lsp {
        CsharpLspOption::CsharpLsp => Value::Bool(false),
        CsharpLspOption::OmniSharp => Value::Bool(true)
    };

    vscode_settings["dotnet.server.path"] = match csharp_lsp {
        CsharpLspOption::CsharpLsp => Value::String("".to_string()),
        CsharpLspOption::OmniSharp => Value::String("omnisharp".to_string())
    };

    let updated_vscode_settings_json = serde_json::to_string(&vscode_settings)?;
    fs::write(&vscode_settings_path, updated_vscode_settings_json)?;

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

/// Adds a C# project to the `.vscode/tasks.json` file.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `project_path` - The path to the C# project.
/// * `project_friendly_name` - The friendly name of the project.
/// * `is_runnable` - Whether the project is runnable.
/// * `is_watchable` - Whether the project is watchable.
/// * `console_utils` - The console utilities.
pub fn add_csharp_project_to_tasks(
    output_directory: &PathBuf,
    project_path: &PathBuf,
    project_friendly_name: &str,
    is_runnable: bool,
    is_watchable: bool,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_tasks_path = output_directory.join(".vscode/tasks.json");

    console_utils.write_info(format!("- 📄 Adding C# project to tasks.json... "))?;

    let vscode_tasks_json = fs::read_to_string(&vscode_tasks_path)?;

    let mut vscode_tasks: Value = serde_json::from_str(&vscode_tasks_json.as_str())?;

    let inputs_node = vscode_tasks["inputs"].as_array_mut().unwrap();

    let csharp_project_input = json!({
        "label": project_friendly_name,
        "value": project_path.to_string_lossy().to_string()
    });

    for input_node_item in inputs_node.iter_mut() {
        if input_node_item["id"] == "projectItem" {
            let input_node_options = input_node_item["options"].as_array_mut().unwrap();

            input_node_options.push(csharp_project_input.clone());
        }

        if is_watchable && input_node_item["id"] == "watchProject" {
            let input_node_options = input_node_item["options"].as_array_mut().unwrap();

            input_node_options.push(csharp_project_input.clone());
        }

        if is_runnable && input_node_item["id"] == "runProject" {
            let input_node_options = input_node_item["options"].as_array_mut().unwrap();

            input_node_options.push(csharp_project_input.clone());
        }
    }

    let updated_vscode_tasks_json = serde_json::to_string_pretty(&vscode_tasks)?;
    fs::write(&vscode_tasks_path, updated_vscode_tasks_json)?;

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

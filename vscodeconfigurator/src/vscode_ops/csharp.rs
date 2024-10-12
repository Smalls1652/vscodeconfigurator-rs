use std::path::PathBuf;

use serde_json::{json, Value};

use crate::{
    console_utils::{ConsoleUtils, OutputEmoji},
    subcommands::csharp::CsharpLspOption,
    vscode_ops::{VSCodeSettingsFile, VSCodeTasksFile}
};

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
    console_utils.write_operation_log("Updating C# LSP option in tasks.json...", OutputEmoji::Document)?;

    let mut vscode_settings = VSCodeSettingsFile::new(output_directory.join(".vscode/settings.json"));

    vscode_settings.values["dotnet.server.useOmnisharp"] = match csharp_lsp {
        CsharpLspOption::CsharpLsp => Value::Bool(false),
        CsharpLspOption::OmniSharp => Value::Bool(true)
    };

    vscode_settings.values["dotnet.server.path"] = match csharp_lsp {
        CsharpLspOption::CsharpLsp => Value::String("".to_string()),
        CsharpLspOption::OmniSharp => Value::String("latest".to_string())
    };

    vscode_settings.write_settings()?;

    console_utils.write_operation_success_log()?;

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
    console_utils.write_operation_log("Adding C# project to tasks.json...", OutputEmoji::Document)?;

    let mut vscode_tasks = VSCodeTasksFile::new(output_directory.join(".vscode/tasks.json"));

    let inputs_node = vscode_tasks.values["inputs"].as_array_mut().unwrap();

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

    vscode_tasks.write_tasks()?;

    console_utils.write_operation_success_log()?;

    Ok(())
}

use std::path::PathBuf;

use serde_json::json;

use crate::{
    console_utils::ConsoleUtils,
    vscode_ops::VSCodeTasks
};


/// Adds a Rust package to the `.vscode/tasks.json` file.
/// 
/// ## Arguments
/// 
/// * `output_directory` - The output directory of the project.
/// * `package_name` - The name of the package.
/// * `package_friendly_name` - The friendly name of the package.
/// * `console_utils` - The console utilities.
pub fn add_package_to_tasks(
    output_directory: &PathBuf,
    package_name: &str,
    package_friendly_name: &str,
    console_utils: &mut ConsoleUtils
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!("- 📄 Adding package to tasks.json... "))?;

    let mut vscode_tasks = VSCodeTasks::new(output_directory.join(".vscode/tasks.json"));

    let inputs_node = vscode_tasks.values["inputs"].as_array_mut().unwrap();

    let package_input = json!({
        "label": package_friendly_name,
        "value": package_name
    });

    for input_node_item in inputs_node.iter_mut() {
        if input_node_item["id"] == "packageName" {
            let input_node_options = input_node_item["options"].as_array_mut().unwrap();

            input_node_options.push(package_input.clone());
        }
    }

    vscode_tasks.write_tasks()?;

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

use std::path::PathBuf;

use serde_json::json;

use crate::{
    logging::{ConsoleLogger, OutputEmoji},
    vscode_ops::VSCodeTasksFile
};

/// Adds a Rust package to the `.vscode/tasks.json` file.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `package_name` - The name of the package.
/// - `package_friendly_name` - The friendly name of the package.
/// - `logger` - The
///   [`ConsoleLogger`](crate::logging::ConsoleLogger) instance
///   for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Adds the package named `my_package` to the `.vscode/tasks.json` file.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let package_name = "my_package";
/// let package_friendly_name = "My Package";
/// let mut logger = ConsoleLogger::new();
///
/// add_package_to_tasks(
///     &output_directory,
///     package_name,
///     package_friendly_name,
///     logger
/// );
/// ```
pub fn add_package_to_tasks(
    output_directory: &PathBuf,
    package_name: &str,
    package_friendly_name: &str,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    logger.write_operation_log("Adding package to tasks.json...", OutputEmoji::Document)?;

    let mut vscode_tasks = VSCodeTasksFile::new(output_directory.join(".vscode/tasks.json"));

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

    logger.write_operation_success_log()?;

    Ok(())
}

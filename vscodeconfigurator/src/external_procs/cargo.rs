use std::{fs, path::PathBuf, process};

use crate::{console_utils::{ConsoleUtils, OutputEmoji}, subcommands::rust::CargoPackageTemplateOption};

/// Initializes a new package with Cargo.
///
/// # Arguments
///
/// - `output_directory` - The output directory for the new solution.
/// - `package_name` - The name of the package.
/// - `package_template` - The type of package to create.
/// - `force` - Whether to forcefully overwrite.
/// - `console_utils` - The [`ConsoleUtils`](crate::console_utils::ConsoleUtils) instance for logging.
/// 
/// # Examples
/// 
/// ## Example 01
/// 
/// Initializes a new package, named `my_package`, with the binary template in `my-project` directory in the temp directory.
/// 
/// ```rust
/// use vscodeconfigurator::{
///    console_utils::ConsoleUtils,
///    subcommands::rust::CargoPackageTemplateOption
/// };
/// 
/// let output_directory = std::env::temp_dir()
///     .join("my-project");
/// let package_name = "my_package";
/// let package_template = CargoPackageTemplateOption::Binary;
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// initalize_package(&output_directory, &package_name, package_template, force, console_utils);
/// ```
pub fn initalize_package(
    output_directory: &PathBuf,
    package_name: &str,
    package_template: CargoPackageTemplateOption,
    force: bool,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_operation_log(
        format!("Initializing package for '{}'... ", package_name).as_str(),
        OutputEmoji::Package
    )?;
    console_utils.save_cursor_position()?;

    let package_output_directory = output_directory
        .join(package_name);

    if package_output_directory.exists() {
        if !force {
            let overwrite_response = console_utils.ask_for_overwrite()?;

            if !overwrite_response {
                
                console_utils.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_dir_all(&package_output_directory)?;  
    }

    let package_template_arg_str = match package_template {
        CargoPackageTemplateOption::Binary => "--bin",
        CargoPackageTemplateOption::Library => "--lib",
    };

    let package_output_directory_string = &package_output_directory
        .to_string_lossy()
        .to_string();

    let cargo_proc_args = vec![
        "init",
        package_template_arg_str,
        "--edition",
        "2021",
        &package_output_directory_string.as_str()
    ];

    process::Command::new("cargo")
        .args(cargo_proc_args)
        .current_dir(output_directory)
        .output()?;

    console_utils.write_operation_success_log()?;

    Ok(())
}

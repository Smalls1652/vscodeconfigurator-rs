use std::{fs, path::PathBuf, process};

use crate::{console_utils::ConsoleUtils, subcommands::rust::init::CargoPackageTemplateOption};

/// Initializes a new package with Cargo.
///
/// ## Arguments
///
/// * `output_directory` - The output directory for the new solution.
/// * `package_name` - The name of the package.
/// * `package_template` - The type of package to create.
pub fn initalize_package(
    output_directory: &PathBuf,
    package_name: &str,
    package_template: CargoPackageTemplateOption,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    console_utils.write_info(format!("- 📦 Initializing package for '{}'... ", package_name))?;
    console_utils.save_cursor_position()?;

    let package_output_directory = output_directory
        .join(package_name);

    if package_output_directory.exists() {
        let overwrite_response = console_utils.ask_for_overwrite()?;

        if !overwrite_response {
            
            console_utils.write_warning(format!("Already exists 🟠\n"))?;
            return Ok(());
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

    console_utils.write_success(format!("Done! ✅\n"))?;

    Ok(())
}

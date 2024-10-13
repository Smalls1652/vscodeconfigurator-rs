use std::{fs, path::PathBuf};

use crate::{console_utils::{ConsoleUtils, OutputEmoji}, utils};

use super::vscode;

/// Copies the `.gitignore` file to the project root.
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
/// Copies the `.gitignore` file to the project root.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///   .join("my-project");
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// copy_gitignore(&output_directory, force, console_utils);
/// ```
pub fn copy_gitignore(
    output_directory: &PathBuf,
    force: bool,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("rust/Git/gitignore");

    let output_file_name = ".gitignore";
    let output_file_path = output_directory.join(&output_file_name);

    console_utils.write_operation_log("Copying '.gitignore' to project root...", OutputEmoji::Document)?;

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

/// Copies the `Cargo.toml` workspace file to the project root.
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
/// Copies the `Cargo.toml` workspace file to the project root.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///  .join("my-project");
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// copy_cargo_workspace_file(&output_directory, force, console_utils);
/// ```
pub fn copy_cargo_workspace_file(
    output_directory: &PathBuf,
    force: bool,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("rust/Cargo/Cargo.workspace.toml");

    let output_file_name = "Cargo.toml";
    let output_file_path = output_directory.join(&output_file_name);

    console_utils.write_operation_log("Copying 'Cargo.toml' to project root...", OutputEmoji::Document)?;

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
///  .join("my-project");
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// copy_vscode_settings(&output_directory, force, console_utils);
/// ```
pub fn copy_vscode_settings(
    output_directory: &PathBuf,
    force: bool,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, console_utils)?;

    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("rust/VSCode/settings.json");

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

    let vscode_settings_json = fs::read_to_string(&template_file_path)?;
    fs::write(&output_file_path, vscode_settings_json)?;

    console_utils.write_operation_success_log()?;

    Ok(())
}

/// Copies the `tasks.json` file to the project root's `.vscode` directory.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `package_name` - The name of the package.
/// - `force` - Whether to forcefully overwrite.
/// - `console_utils` - The [`ConsoleUtils`](crate::console_utils::ConsoleUtils) instance for logging.
/// 
/// # Examples
/// 
/// ## Example 01
/// 
/// Copies the `tasks.json` file to the project root's `.vscode` directory with the package name `my_package`.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///     .join("my-project");
/// let package_name = "my_package";
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// copy_vscode_tasks(&output_directory, &package_name, force, console_utils);
/// ```
pub fn copy_vscode_tasks(
    output_directory: &PathBuf,
    package_name: &str,
    force: bool,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, console_utils)?;

    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("rust/VSCode/tasks.json");

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

    let vscode_tasks_json = fs::read_to_string(&template_file_path)?
        .replace("{{basePackageName}}", package_name);

    fs::write(&output_file_path, vscode_tasks_json)?;

    console_utils.write_operation_success_log()?;

    Ok(())
}

/// Creates the `tools` directory in the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `console_utils` - The [`ConsoleUtils`](crate::console_utils::ConsoleUtils) instance for logging.
/// 
/// # Examples
/// 
/// ## Example 01
/// 
/// Creates the `tools` directory in the project root.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///    .join("my-project");
/// let mut console_utils = ConsoleUtils::new();
/// 
/// ensure_tools_dir_exists(&output_directory, console_utils);
/// ```
pub fn ensure_tools_dir_exists(
    output_directory: &PathBuf,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_dir_path = output_directory.join("tools");

    if !vscode_dir_path.exists() {
        console_utils.write_operation_log("Creating 'tools' directory...", OutputEmoji::Folder)?;
        std::fs::create_dir(&vscode_dir_path)?;
        console_utils.write_operation_success_log()?;
    }

    Ok(())
}

/// Copies the `Build-Package.ps1` file to the tools dir.
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
/// Copy the `Build-Package.ps1` file to the tools directory.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///   .join("my-project");
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// copy_build_pwsh_script(&output_directory, force, console_utils);
/// ```
pub fn copy_build_pwsh_script(
    output_directory: &PathBuf,
    force: bool,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_tools_dir_exists(output_directory, console_utils)?;
    
    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("rust/Tools/Build-Package.ps1");

    let output_file_name = "Build-Package.ps1";
    let output_file_path = output_directory.join("tools").join(&output_file_name);

    console_utils.write_operation_log("Copying 'Build-Package.ps1' to tools directory... ", OutputEmoji::Document)?;

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

/// Copies the `Clean-Package.ps1` file to the tools dir.
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
/// Copy the `Clean-Package.ps1` file to the tools directory.
/// 
/// ```rust
/// use vscodeconfigurator::console_utils::ConsoleUtils;
/// 
/// let output_directory = std::env::temp_dir()
///  .join("my-project");
/// let force = false;
/// let mut console_utils = ConsoleUtils::new();
/// 
/// copy_clean_pwsh_script(&output_directory, force, console_utils);
/// ```
pub fn copy_clean_pwsh_script(
    output_directory: &PathBuf,
    force: bool,
    console_utils: &mut ConsoleUtils,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_tools_dir_exists(output_directory, console_utils)?;

    let core_templates_path = utils::get_core_templates_path();
    let template_file_path = core_templates_path.join("rust/Tools/Clean-Package.ps1");

    let output_file_name = "Clean-Package.ps1";
    let output_file_path = output_directory.join("tools").join(&output_file_name);

    console_utils.write_operation_log("Copying 'Clean-Package.ps1' to tools directory... ", OutputEmoji::Document)?;

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

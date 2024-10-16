use std::{borrow::Borrow, fs, path::PathBuf};

use super::{vscode, TemplateFile};
use crate::logging::{ConsoleLogger, OutputEmoji};

/// Copies the `GitVersion.yml` file to the project root.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](crate::logging::ConsoleLogger) instance
///   for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Copies the `GitVersion.yml` file to the project root.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// csharp_copy_gitversion(&output_directory, force, logger);
/// ```
pub fn csharp_copy_gitversion(
    output_directory: &PathBuf,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    let template_file = TemplateFile::new(
        "csharp/GitVersion/GitVersion.yml",
        output_directory,
        "GitVersion.yml"
    );

    logger.write_operation_log(
        format!(
            "Copying '{}' to project root...",
            &template_file.output_file_name
        )
        .as_str(),
        OutputEmoji::Document
    )?;

    if template_file.output_file_exists {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }
    }

    template_file.copy_file()?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Copies the `settings.json` file to the project root's `.vscode` directory.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `solution_name` - The name of the solution.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](crate::logging::ConsoleLogger) instance
///   for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Copies the `settings.json` file to the project root's `.vscode` directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let solution_name = "MySolution".to_string();
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// csharp_copy_vscode_settings(&output_directory, &solution_name, force, logger);
/// ```
pub fn csharp_copy_vscode_settings(
    output_directory: &PathBuf,
    solution_name: &String,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, logger)?;

    let template_file = TemplateFile::new(
        "csharp/VSCode/settings.json",
        output_directory.join(".vscode").borrow(),
        "settings.json"
    );

    logger.write_operation_log(
        format!(
            "Copying '{}' to '.vscode' directory...",
            &template_file.output_file_name
        )
        .as_str(),
        OutputEmoji::Document
    )?;

    if template_file.output_file_exists {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(&template_file.output_file_path)?;
    }

    let solution_name_full = format!("{:}.sln", solution_name);
    let vscode_settings_json = fs::read_to_string(&template_file.template_file_path)?
        .replace("{{solutionName}}", &solution_name_full);

    fs::write(&template_file.output_file_path, vscode_settings_json)?;

    logger.write_operation_success_log()?;

    Ok(())
}

/// Copies the `tasks.json` file to the project root's `.vscode` directory.
///
/// # Arguments
///
/// - `output_directory` - The output directory of the project.
/// - `solution_name` - The name of the solution.
/// - `force` - Whether to forcefully overwrite.
/// - `logger` - The [`ConsoleLogger`](crate::logging::ConsoleLogger) instance
///   for logging.
///
/// # Examples
///
/// ## Example 01
///
/// Copies the `tasks.json` file to the project root's `.vscode` directory.
///
/// ```rust
/// use vscodeconfigurator::logger::ConsoleLogger;
///
/// let output_directory = std::env::temp_dir().join("my-project");
/// let solution_name = "MySolution".to_string();
/// let force = false;
/// let mut logger = ConsoleLogger::new();
///
/// csharp_copy_vscode_tasks(&output_directory, &solution_name, force, logger);
/// ```
pub fn csharp_copy_vscode_tasks(
    output_directory: &PathBuf,
    solution_name: &String,
    force: bool,
    logger: &mut ConsoleLogger
) -> Result<(), Box<dyn std::error::Error>> {
    vscode::ensure_vscode_dir_exists(output_directory, logger)?;

    let template_file = TemplateFile::new(
        "csharp/VSCode/tasks.json",
        output_directory.join(".vscode").borrow(),
        "tasks.json"
    );

    logger.write_operation_log(
        format!(
            "Copying '{}' to '.vscode' directory...",
            &template_file.output_file_name
        )
        .as_str(),
        OutputEmoji::Document
    )?;

    if template_file.output_file_exists {
        if !force {
            let overwrite_response = logger.ask_for_overwrite()?;

            if !overwrite_response {
                logger.write_warning(format!("Already exists 🟠\n"))?;
                return Ok(());
            }
        }

        fs::remove_file(&template_file.output_file_path)?;
    }

    let solution_name_full = format!("{:}.sln", solution_name);
    let vscode_tasks_json = fs::read_to_string(&template_file.template_file_path)?
        .replace("{{solutionName}}", &solution_name_full);

    fs::write(&template_file.output_file_path, vscode_tasks_json)?;

    logger.write_operation_success_log()?;

    Ok(())
}

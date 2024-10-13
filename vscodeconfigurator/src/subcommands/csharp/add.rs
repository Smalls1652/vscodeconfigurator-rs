use std::{env, io::ErrorKind, path::PathBuf, process};

use clap::{Args, ValueHint};
use vscodeconfigurator_lib::logging::ConsoleLogger;

use crate::{external_procs::dotnet, vscode_ops};

/// Defines the arguments for the `csharp add` command and the logic to run the
/// command.
#[derive(Args, Debug, PartialEq)]
pub struct AddCommandArgs {
    /// The solution file to add the project to.
    #[arg(
        long = "solution-file-path",
        required = false,
        value_hint = ValueHint::FilePath
    )]
    solution_file_path: Option<PathBuf>,

    /// The path to the project.
    #[arg(
        long = "project-path",
        required = true,
        value_hint = ValueHint::AnyPath
    )]
    project_path: PathBuf,

    /// The friendly name of the project.
    #[arg(
        long = "project-friendly-name",
        required = false,
        value_hint = ValueHint::Other
    )]
    project_friendly_name: Option<String>,

    /// Whether the project is runnable.
    #[arg(long = "is-runnable", required = false, default_value = "false")]
    is_runnable: bool,

    /// Whether the project is watchable.
    #[arg(long = "is-watchable", required = false, default_value = "false")]
    is_watchable: bool
}

impl AddCommandArgs {
    /// Runs the `csharp add` command.
    pub fn run_command(
        &self,
        logger: &mut ConsoleLogger
    ) -> Result<(), Box<dyn std::error::Error>> {
        let solution_file_path: PathBuf;

        if self.solution_file_path.is_none() {
            solution_file_path = match get_solution_file_path_default_value() {
                Ok(path) => path,
                Err(_) => {
                    logger
                        .write_error(format!("No solution file found in the current directory."))?;

                    process::exit(1);
                }
            };
        } else {
            solution_file_path = self.solution_file_path.clone().unwrap();
            if !&solution_file_path.exists() {
                logger.write_error(format!(
                    "The solution file path '{}' does not exist.",
                    &solution_file_path.display()
                ))?;

                process::exit(1);
            }
        }

        if !&self.project_path.exists() {
            logger.write_error(format!(
                "The project path '{}' does not exist.",
                &self.project_path.display()
            ))?;

            process::exit(1);
        }

        let project_friendly_name: String;

        if self.project_friendly_name.is_none() {
            project_friendly_name = match get_project_friendly_name(&self.project_path) {
                Ok(name) => name,
                Err(_) => {
                    logger
                        .write_error(format!("No project file found in the project directory."))?;

                    process::exit(1);
                }
            };
        } else {
            project_friendly_name = self.project_friendly_name.clone().unwrap();
        }

        logger.write_operation_category("Add project")?;
        dotnet::add_project_to_solution(&solution_file_path, &self.project_path, logger)?;
        vscode_ops::csharp::add_csharp_project_to_tasks(
            &PathBuf::from(solution_file_path.parent().unwrap()),
            &self.project_path,
            &project_friendly_name,
            self.is_runnable,
            self.is_watchable,
            logger
        )?;

        Ok(())
    }
}

/// Gets the default value for the `solution_file_path` (`--solution-file-path`)
/// argument if it is not provided by the user.
fn get_solution_file_path_default_value() -> Result<PathBuf, ErrorKind> {
    let current_dir = env::current_dir().unwrap();

    for file_item in current_dir.read_dir().unwrap() {
        let file_item = file_item.unwrap();

        let file_path = file_item.path();

        if file_path.extension().is_none() {
            continue;
        }

        if file_path.extension().unwrap() == "sln" {
            return Ok(PathBuf::from(file_path));
        }
    }

    Err(ErrorKind::NotFound)
}

/// Gets the friendly name of the project, if it is not provided by the user.
fn get_project_friendly_name(project_path: &PathBuf) -> Result<String, ErrorKind> {
    for file_item in project_path.read_dir().unwrap() {
        let file_item = file_item.unwrap();

        let file_path = file_item.path();

        if file_path.extension().is_none() {
            continue;
        }

        if file_path.extension().unwrap() == "csproj" {
            return Ok(file_path.file_stem().unwrap().to_str().unwrap().to_string());
        }
    }

    Err(ErrorKind::NotFound)
}

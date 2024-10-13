use clap::{builder::TypedValueParser, Args, ValueEnum, ValueHint};
use vscodeconfigurator_lib::{io::OutputDirectory, logging::ConsoleLogger};

use crate::{
    external_procs::{cargo, git},
    template_ops
};

/// Defines the arguments for the `rust init` command and the logic to run the
/// command.
#[derive(Args, Debug, PartialEq)]
pub struct RustInitCommandArgs {
    /// The output directory for the new project.
    #[arg(
        short = 'o',
        long = "output-directory",
        required = false,
        value_parser = clap::builder::OsStringValueParser::new().map(|s| OutputDirectory::from_os_string(s).unwrap()),
        default_value = OutputDirectory::from_current_dir(),
        value_hint = ValueHint::DirPath
    )]
    output_directory: OutputDirectory,

    /// The name of the base package.
    #[arg(short = 'n', long = "base-package-name", required = true)]
    base_package_name: String,

    /// The type of Cargo package to create.
    #[arg(
        long = "base-package-template",
        required = false,
        value_enum,
        default_value = "Library"
    )]
    base_package_template: CargoPackageTemplateOption,

    /// Force the command to run without prompting for confirmation.
    #[arg(short = 'f', long = "force", required = false, default_value = "false")]
    force: bool
}

impl RustInitCommandArgs {
    /// Runs the `run init` command.
    pub fn run_command(
        &self,
        logger: &mut ConsoleLogger
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut output_directory = self.output_directory.clone();

        output_directory = output_directory
            .resolve_home_dir()?
            .trim_trailing_slashes()?;

        output_directory.create_if_not_exists()?;

        let output_directory_absolute = output_directory.to_absolute();

        logger.write_operation_category("Git")?;
        git::initialize_git_repo(&output_directory_absolute, logger)?;
        template_ops::rust::copy_gitignore(&output_directory_absolute, self.force, logger)?;
        logger.write_newline()?;

        logger.write_operation_category("VSCode")?;
        template_ops::rust::copy_vscode_settings(&output_directory_absolute, self.force, logger)?;
        template_ops::rust::copy_vscode_tasks(
            &output_directory_absolute,
            &self.base_package_name,
            self.force,
            logger
        )?;
        template_ops::rust::copy_build_pwsh_script(&output_directory_absolute, self.force, logger)?;
        template_ops::rust::copy_clean_pwsh_script(&output_directory_absolute, self.force, logger)?;
        logger.write_newline()?;

        logger.write_operation_category("Rust")?;
        template_ops::rust::copy_cargo_workspace_file(
            &output_directory_absolute,
            self.force,
            logger
        )?;
        cargo::initalize_package(
            &output_directory_absolute,
            &self.base_package_name,
            self.base_package_template,
            self.force,
            logger
        )?;
        logger.write_newline()?;

        logger.write_project_initialized_log()?;

        Ok(())
    }
}

/// The type of Cargo package template to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CargoPackageTemplateOption {
    /// A binary.
    #[value(name = "Binary")]
    Binary,

    /// A library.
    #[value(name = "Library")]
    Library
}

use clap::{builder::TypedValueParser, Args, ValueEnum, ValueHint};

use crate::{console_utils::ConsoleUtils, external_procs::git, io::OutputDirectory, template_ops};

/// Defines the arguments for the `rust init` command and the logic to run the command.
#[derive(Args, Debug, PartialEq)]
pub struct RustInitCommandArgs {
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
    #[arg(
        short = 'n',
        long = "base-package-name",
        required = true
    )]
    base_package_name: String,

    /// The type of Cargo package to create.
    #[arg(
        long = "base-package-template",
        required = false,
        value_enum,
        default_value = "Library"
    )]
    base_package_template: CargoPackageTemplateOption
}

impl RustInitCommandArgs {
    /// Runs the `run init` command.
    pub fn run_command(&self, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
        let mut output_directory = self.output_directory.clone();

        output_directory = output_directory
            .resolve_home_dir()?
            .trim_trailing_slashes()?;

        output_directory.create_if_not_exists()?;

        let output_directory_absolute = output_directory.to_absolute();

        console_utils.write_info(format!("\n🚀 Git\n"))?;
        git::initialize_git_repo(&output_directory_absolute, console_utils)?;
        template_ops::rust::copy_gitignore(&output_directory_absolute, console_utils)?;

        console_utils.write_info(format!("\n🚀 VSCode\n"))?;
        template_ops::rust::copy_vscode_settings(&output_directory_absolute, console_utils)?;
        template_ops::rust::copy_vscode_tasks(&output_directory_absolute, console_utils)?;

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
    Library,
}

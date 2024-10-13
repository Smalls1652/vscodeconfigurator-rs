use clap::{builder::TypedValueParser, Args, ValueHint};
use vscodeconfigurator_lib::{
    error::{CliError, CliErrorKind},
    io::OutputDirectory,
    logging::ConsoleLogger
};

use crate::vscode_ops;

/// Defines the arguments for the `rust add` command and the logic to run the
/// command.
#[derive(Args, Debug, PartialEq)]
pub struct RustAddCommandArgs {
    /// The output directory for the project.
    #[arg(
        short = 'o',
        long = "output-directory",
        required = false,
        value_parser = clap::builder::OsStringValueParser::new().map(|s| OutputDirectory::from_os_string(s).unwrap()),
        default_value = OutputDirectory::from_current_dir(),
        value_hint = ValueHint::DirPath
    )]
    output_directory: OutputDirectory,

    /// The name of the package.
    #[arg(
        long = "package-name",
        required = true,
        value_hint = ValueHint::AnyPath
    )]
    package_name: String,

    /// The friendly name of the package.
    #[arg(
        long = "package-friendly-name",
        required = false,
        value_hint = ValueHint::Other
    )]
    package_friendly_name: Option<String>
}

impl RustAddCommandArgs {
    /// Runs the `run add` command.
    pub fn run_command(
        &self,
        logger: &mut ConsoleLogger
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut output_directory = self.output_directory.clone();

        output_directory = output_directory
            .resolve_home_dir()?
            .trim_trailing_slashes()?;

        if !output_directory.as_pathbuf().exists() {
            return Err(CliError::new(
                "The specified output directory does not exist.",
                CliErrorKind::OutputDirectoryDoesNotExist
            )
            .into());
        }

        let output_directory_absolute = output_directory.to_absolute();

        let package_friendly_name = match &self.package_friendly_name {
            Some(ref name) => &name.as_str(),
            None => self.package_name.as_str()
        };

        logger.write_operation_category("Add package")?;
        vscode_ops::rust::add_package_to_tasks(
            &output_directory_absolute,
            self.package_name.as_str(),
            package_friendly_name,
            logger
        )?;

        Ok(())
    }
}

use clap::{builder::TypedValueParser, Args, ValueEnum, ValueHint};

use crate::{
    console_utils::ConsoleUtils, error::{CliError, CliErrorKind}, external_procs::{dotnet, git}, io::OutputDirectory, template_ops, vscode_ops
};

/// Defines the arguments for the `csharp init` command and the logic to run the command.
#[derive(Args, Debug, PartialEq)]
pub struct InitCommandArgs {
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

    /// The name of the solution file.
    /// 
    /// If not provided, the name of the output directory is used.
    #[arg(
        short = 'n',
        long = "solution-name",
        required = false,
        value_hint = ValueHint::Other
    )]
    solution_name: Option<String>,

    /// Add GitVersion to the project.
    #[arg(
        long = "add-gitversion",
        required = false,
        default_value = "false"
    )]
    add_gitversion: bool,

    /// Add a NuGet configuration file to the project.
    #[arg(
        long = "add-nuget-config",
        required = false,
        default_value = "false"
    )]
    add_nuget_config: bool,

    /// Whether to enable centrally managed packages.
    #[arg(
        long = "enable-centrally-managed-packages",
        required = false,
        default_value = "false"
    )]
    enable_centrally_managed_packages: bool,

    /// The C# language server to use.
    #[arg(
        long = "csharp-lsp",
        required = false,
        value_enum,
        default_value = "CsharpLsp"
    )]
    csharp_lsp: CsharpLspOption,
}

impl InitCommandArgs {
    /// Runs the `csharp init` command.
    pub fn run_command(&self, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn std::error::Error>> {
        let mut output_directory = self.output_directory.clone();

        output_directory = output_directory
            .resolve_home_dir()?
            .trim_trailing_slashes()?;

        output_directory.create_if_not_exists()?;

        let output_directory_absolute = output_directory.to_absolute();

        let parsed_solution_name = self.get_solution_name_value();

        if parsed_solution_name.is_none() {
            return Err(
                CliError::new(
                    "The solution name could not be determined.", 
                    CliErrorKind::UnableToParseSolutionName).into()
            );
        }

        let solution_name = parsed_solution_name.unwrap();

        console_utils.write_info(format!("🚀 Basic\n"))?;
        dotnet::add_dotnet_globaljson(&output_directory_absolute, console_utils)?;

        console_utils.write_info(format!("\n🚀 Git\n"))?;
        git::initialize_git_repo(&output_directory_absolute, console_utils)?;
        dotnet::add_dotnet_gitignore(&output_directory_absolute, console_utils)?;

        console_utils.write_info(format!("\n🚀 .NET\n"))?;
        dotnet::initalize_dotnet_solution(&output_directory_absolute, &solution_name, console_utils)?;
        dotnet::add_dotnet_buildprops(&output_directory_absolute, console_utils)?;

        if self.add_nuget_config {
            dotnet::add_dotnet_nugetconfig(&output_directory_absolute, console_utils)?;
        }

        if self.enable_centrally_managed_packages {
            dotnet::add_dotnet_packagesprops(&output_directory_absolute, console_utils)?;
        }

        if self.add_gitversion {
            console_utils.write_info(format!("\n🚀 GitVersion\n"))?;
            dotnet::add_dotnet_tool(&output_directory_absolute, "GitVersion.Tool", console_utils)?;
            template_ops::csharp::csharp_copy_gitversion(&output_directory_absolute, console_utils)?;
        }

        console_utils.write_info(format!("\n🚀 VSCode\n"))?;
        template_ops::csharp::csharp_copy_vscode_settings(&output_directory_absolute, &solution_name, console_utils)?;
        vscode_ops::csharp::update_csharp_lsp(&output_directory_absolute, self.csharp_lsp, console_utils)?;
        template_ops::csharp::csharp_copy_vscode_tasks(&output_directory_absolute, &solution_name, console_utils)?;

        console_utils.write_info(format!("\n🥳 VSCode project initialized!\n"))?;

        Ok(())
    }

    /// Gets the default value for the `solution_name` (`--solution-name`) argument if
    /// it is not provided by the user.
    fn get_solution_name_value(&self) -> Option<String> {
        match &self.solution_name.is_none() {
            true => {
                let output_directory_pathbuf = &self.output_directory
                    .as_pathbuf();

                let output_directory_name = output_directory_pathbuf
                    .file_name();

                match output_directory_name {
                    Some(name) => Some(
                        name
                            .to_string_lossy()
                            .to_string()
                    ),
                    None => None
                }
            }
            
            false => Some(self.solution_name
                .as_ref()
                .unwrap()
                .clone())
        }
    }
}

/// The type of C# language server to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CsharpLspOption {
    /// The C# language server provided by the C# extension for Visual Studio Code.
    #[value(name = "CsharpLsp")]
    CsharpLsp,

    /// The original C# language server provided by OmniSharp.
    #[value(name = "OmniSharp")]
    OmniSharp,
}

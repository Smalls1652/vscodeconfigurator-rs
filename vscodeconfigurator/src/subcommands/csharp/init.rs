use std::{env, path::{absolute, Path, PathBuf}};
use clap::{Args, builder::TypedValueParser, ValueEnum};

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(
        short = 'o',
        long = "output-directory",
        help = "The output directory for the new project.",
        required = false,
        value_parser = clap::builder::OsStringValueParser::new().map(|s| PathBuf::from(s)),
        default_value = env::current_dir().unwrap().into_os_string()
    )]
    output_directory: PathBuf,

    #[arg(
        short = 'n',
        long = "solution-name",
        help = "The default solution file.",
        required = false,
        default_value = env::current_dir().unwrap().file_name().unwrap().to_str().unwrap().to_string()
    )]
    solution_name: String,

    #[arg(
        long = "add-gitversion",
        help = "Add GitVersion to the project.",
        required = false,
        default_value = "false"
    )]
    add_gitversion: bool,

    #[arg(
        long = "add-nuget-config",
        help = "Add a NuGet configuration file to the project.",
        required = false,
        default_value = "false"
    )]
    add_nuget_config: bool,

    #[arg(
        long = "csharp-lsp",
        help = "The C# language server to use.",
        required = false,
        value_enum,
        default_value = "CsharpLsp"
    )]
    csharp_lsp: CsharpLspOption
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CsharpLspOption {
    /// The C# language server provided by the C# extension for Visual Studio Code.
    CsharpLsp,

    #[value(name = "OmniSharp")]
    /// The original C# language server provided by OmniSharp.
    OmniSharp
}

pub fn run_command(init_args: &InitArgs) {
    println!("Running csharp init command");

    let mut output_directory = init_args.output_directory.clone();

    if init_args.output_directory.starts_with("~") {
        println!("Output directory starts with ~");

        let home_dir_env_var_key = match env::consts::OS {
            "windows" => "USERPROFILE",
            "unix" | "macos" => "HOME",
            _ => panic!("Unsupported OS")
        };
        let home_dir_env_var = env::var(home_dir_env_var_key).unwrap();
        let home_dir = Path::new(&home_dir_env_var);

        output_directory = PathBuf::from(&home_dir).join(output_directory.strip_prefix("~").unwrap());
    }

    let output_directory_canonicalized = absolute(output_directory).unwrap();

    println!("Output directory: {:?}", output_directory_canonicalized.to_string_lossy());

    let solution_name = init_args.solution_name.clone();

    let solution_file_name = format!("{}.sln", solution_name);

    println!("Solution name: {}", solution_name);
    println!("Solution file name: {}", solution_file_name);

    let add_gitversion = init_args.add_gitversion;
    let add_nuget_config = init_args.add_nuget_config;
    let csharp_lsp = init_args.csharp_lsp;

    println!("Add GitVersion: {}", add_gitversion);
    println!("Add NuGet configuration: {}", add_nuget_config);
    println!("C# language server: {:?}", csharp_lsp);
}

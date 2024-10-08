mod console_utils;
mod error;
mod external_procs;
mod io;
mod subcommands;
mod template_ops;
mod utils;
mod vscode_ops;

use std::{error::Error, process};

use clap::{crate_version, CommandFactory, Parser};
use clap_complete::aot::generate;

use self::{
    console_utils::ConsoleUtils,
    error::{CliError, CliErrorKind},
    subcommands::RootSubcommands
};

/// The main CLI struct.
#[derive(Parser, Debug, PartialEq)]
#[command(
    name = "VSCode Configurator",
    bin_name = "vscodeconfigurator",
    version = crate_version!(),
    about = "Quickly bootstrap and manage projects for VSCode.",
    long_about = None,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<RootSubcommands>,
}

/// The entry point for the CLI.
fn main() -> Result<(), Box<dyn Error>> {
    let mut console_utils = ConsoleUtils::new(None);

    let cli = Cli::parse();

    let result = match &cli.command {
        Some(RootSubcommands::Completions(command)) => {
            generate(command.shell, &mut Cli::command(), "vscodeconfigurator", &mut std::io::stdout());
            Ok(())
        }

        Some(RootSubcommands::Csharp { command }) =>
            command
                .as_ref()
                .unwrap()
                .match_subcommand(&mut console_utils),

        Some(RootSubcommands::Rust { command }) =>
            command
                .as_ref()
                .unwrap()
                .match_subcommand(&mut console_utils),

        None => Err(CliError::new("No subcommand provided.", CliErrorKind::NoSubcommandProvided).into())
    };

    match result {
        Err(e) => {
            console_utils.write_error_extended(e)?;
            process::exit(1);
        }

        Ok(_) => Ok(())
    }
}

mod subcommands;
mod external_procs;
mod console_utils;
mod template_ops;
mod vscode_ops;
mod utils;

use clap::{crate_version, Parser};
use subcommands::RootSubcommands;

/// The main CLI struct.
#[derive(Parser)]
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
    command: Option<RootSubcommands>
}

/// The entry point for the CLI.
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(RootSubcommands::Csharp { command }) =>
            command
                .as_ref()
                .unwrap()
                .match_subcommand(),

        None => println!("No subcommand provided")
    }
}

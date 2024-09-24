mod subcommands;

use clap::{crate_version, Parser};
use subcommands::{csharp, RootSubcommands};

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(RootSubcommands::Csharp { command }) => csharp::match_subcommand(command),

        None => println!("No subcommand provided")
    }
}

pub mod csharp;

use clap::Subcommand;
use csharp::CsharpSubcommands;

/// The root subcommands for the CLI.
#[derive(Subcommand)]
pub enum RootSubcommands {
    /// Subcommands for C# projects.
    Csharp {
        #[command(subcommand)]
        command: Option<CsharpSubcommands>
    }
}

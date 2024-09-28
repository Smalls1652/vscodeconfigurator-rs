pub mod csharp;

use clap::{Subcommand, Args};
use clap_complete::Shell;
use csharp::CsharpSubcommands;

/// The root subcommands for the CLI.
#[derive(Subcommand, Debug, PartialEq)]
pub enum RootSubcommands {
    /// Subcommands for C# projects.
    Csharp {
        #[command(subcommand)]
        command: Option<CsharpSubcommands>
    },

    /// Generate completion scripts for the shell of your choice.
    Completions(Completions)
}

/// Generate completion scripts for the shell of your choice.
#[derive(Args, Debug, PartialEq)]
pub struct Completions {
    /// The shell to generate completions for.
    pub shell: Shell
}

pub mod csharp;
pub mod rust;

use clap::{Args, Subcommand};
use clap_complete::Shell;
use csharp::CsharpSubcommands;
use rust::RustSubcommands;

/// The root subcommands for the CLI.
#[derive(Subcommand, Debug, PartialEq)]
pub enum RootSubcommands {
    /// Subcommands for C# projects.
    Csharp {
        #[command(subcommand)]
        command: Option<CsharpSubcommands>
    },

    /// Subcommands for Rust projects.
    Rust {
        #[command(subcommand)]
        command: Option<RustSubcommands>
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

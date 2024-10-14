pub mod csharp;
pub mod rust;

use clap::{Args, Subcommand};
use clap_complete::Shell;
use csharp::CsharpSubcommands;
use rust::RustSubcommands;
use vscodeconfigurator_lib::logging::ConsoleLogger;

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

/// A trait for subcommands used in the VSCode Configurator CLI.
pub trait ConfiguratorSubcommand {
    /// Matches the subcommand provided by the user and runs the corresponding
    /// command.
    fn match_subcommand(&self, logger: &mut ConsoleLogger) -> Result<(), Box<dyn std::error::Error>>;
}

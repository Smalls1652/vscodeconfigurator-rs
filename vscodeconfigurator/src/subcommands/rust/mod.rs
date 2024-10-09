mod add;
mod init;

pub use init::*;
pub use add::*;

use std::error::Error;

use clap::Subcommand;

use crate::console_utils::ConsoleUtils;

/// Subcommands for Rust projects.
#[derive(Subcommand, Debug, PartialEq)]
#[command(
    about = "Commands for creating and managing Rust projects.",
    arg_required_else_help = true,
)]
pub enum RustSubcommands {
    /// Initialize a new Rust project.
    #[command(
        about = "Initialize a new Rust project.",
        long_about = None
    )]
    Init(RustInitCommandArgs),

    /// Add a new package to a Rust project.
    #[command(
        about = "Add a new package to a Rust project.",
        long_about = None
    )]
    Add(RustAddCommandArgs),
}

impl RustSubcommands {
    /// Matches the subcommand provided by the user and runs the corresponding command.
    pub fn match_subcommand(&self, console_utils: &mut ConsoleUtils) -> Result<(), Box<dyn Error>> {
        match self {
            RustSubcommands::Init(init_args) =>
                init_args.run_command(console_utils)?,

            RustSubcommands::Add(add_args) =>
                add_args.run_command(console_utils)?            
        };

        Ok(())
    }
}

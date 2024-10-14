mod add;
mod init;

use std::error::Error;

use clap::Subcommand;
use vscodeconfigurator_lib::logging::ConsoleLogger;

use self::{add::RustAddCommandArgs, init::RustInitCommandArgs};
use crate::subcommands::ConfiguratorSubcommand;

/// Subcommands for Rust projects.
#[derive(Subcommand, Debug, PartialEq)]
#[command(
    about = "Commands for creating and managing Rust projects.",
    arg_required_else_help = true
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
    Add(RustAddCommandArgs)
}

impl ConfiguratorSubcommand for RustSubcommands {
    /// Matches the subcommand provided by the user and runs the corresponding
    /// command.
    fn match_subcommand(
        &self,
        logger: &mut ConsoleLogger
    ) -> Result<(), Box<dyn Error>> {
        match self {
            RustSubcommands::Init(init_args) => init_args.run_command(logger)?,

            RustSubcommands::Add(add_args) => add_args.run_command(logger)?
        };

        Ok(())
    }
}

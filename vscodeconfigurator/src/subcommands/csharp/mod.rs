mod add;
mod init;

use std::error::Error;

use clap::Subcommand;
use vscodeconfigurator_lib::logging::ConsoleLogger;

use self::{add::AddCommandArgs, init::InitCommandArgs};

/// Subcommands for C# projects.
#[derive(Subcommand, Debug, PartialEq)]
#[command(
    about = "Commands for creating and managing C# projects.",
    arg_required_else_help = true
)]
pub enum CsharpSubcommands {
    /// Initialize a new C# project.
    #[command(
        about = "Initialize a new C# project.",
        long_about = None
    )]
    Init(InitCommandArgs),

    /// Add a new project to a C# solution.
    #[command(
        about = "Add a new project to a C# solution.",
        long_about = None
    )]
    Add(AddCommandArgs)
}

impl CsharpSubcommands {
    /// Matches the subcommand provided by the user and runs the corresponding
    /// command.
    pub fn match_subcommand(
        &self,
        logger: &mut ConsoleLogger
    ) -> Result<(), Box<dyn Error>> {
        match self {
            CsharpSubcommands::Init(init_args) => InitCommandArgs::run_command(init_args, logger)?,

            CsharpSubcommands::Add(add_args) => AddCommandArgs::run_command(add_args, logger)?
        };

        Ok(())
    }
}

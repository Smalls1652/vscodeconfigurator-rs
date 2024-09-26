pub mod init;

use clap::Subcommand;
use init::InitCommandArgs;

/// Subcommands for C# projects.
#[derive(Subcommand)]
#[command(
    about = "Commands for creating and managing C# projects.",
    arg_required_else_help = true,
)]
pub enum CsharpSubcommands {
    /// Initialize a new C# project.
    #[command(
        about = "Initialize a new C# project.",
        long_about = None
    )]
    Init(InitCommandArgs)
}

impl CsharpSubcommands {
    /// Matches the subcommand provided by the user and runs the corresponding command.
    pub fn match_subcommand(&self) {
        match self {
            CsharpSubcommands::Init(init_args) =>
                InitCommandArgs::run_command(init_args)
                    .expect("Failed to run 'csharp init' command.")
        }
    }
}

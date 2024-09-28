pub mod init;
pub mod add;

use clap::Subcommand;
use init::InitCommandArgs;
use add::AddCommandArgs;

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
    Init(InitCommandArgs),

    /// Add a new project to a C# solution.
    #[command(
        about = "Add a new project to a C# solution.",
        long_about = None
    )]
    Add(AddCommandArgs)
}

impl CsharpSubcommands {
    /// Matches the subcommand provided by the user and runs the corresponding command.
    pub fn match_subcommand(&self) {
        match self {
            CsharpSubcommands::Init(init_args) =>
                InitCommandArgs::run_command(init_args)
                    .expect("Failed to run 'csharp init' command."),

            CsharpSubcommands::Add(add_args) =>
                AddCommandArgs::run_command(add_args)
                    .expect("Failed to run 'csharp add' command.")
        }
    }
}

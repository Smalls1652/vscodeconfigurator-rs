pub mod init;

use clap::Subcommand;

#[derive(Subcommand)]
#[command(
    about = "Commands for creating and managing C# projects.",
    arg_required_else_help = true,
)]
pub enum CsharpSubcommands {
    #[command(
        about = "Initialize a new C# project.",
        long_about = "Initialize a new C# project."
    )]
    Init(init::InitArgs)
}

pub fn match_subcommand(subcommand: &Option<CsharpSubcommands>) {
    match subcommand {
        Some(CsharpSubcommands::Init(init_args)) => init::run_command(init_args),
        None => println!("No csharp subcommand provided")
    }
}

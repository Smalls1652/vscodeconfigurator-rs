pub mod csharp;

use clap::Subcommand;
use csharp::CsharpSubcommands;

#[derive(Subcommand)]
pub enum RootSubcommands {
    Csharp {
        #[command(subcommand)]
        command: Option<CsharpSubcommands>
    }
}

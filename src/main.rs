use clap::{Parser, Subcommand};
use sword_cli::commands::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Run {
        #[command(subcommand)]
        mode: Option<RunMode>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create => {
            CreateAppExecutor::execute(CreateAppInput::collect()).await?
        }
        Commands::Run { mode } => RunAppExecutor::execute(mode.clone()).await?,
    }

    Ok(())
}

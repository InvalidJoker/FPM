use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

mod cli;
mod manager;

use cli::Cli;
use manager::Manager;

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_ansi(true)
        .init();

    let cli = Cli::parse();
    let manager = Manager::new().await?;

    cli::handle_command(cli.cmd, manager).await
}

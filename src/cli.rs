use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::sync::Arc;
use tracing::error;

use crate::manager::Manager;

#[derive(Parser)]
#[command(name = "fpm")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start { target: String },
    Stop { name: String },
    Restart { name: String },
    List,
    Status { name: Option<String> },
    Logs { name: String, lines: Option<usize> },
    Daemon,
}

pub async fn handle_command(cmd: Commands, manager: Arc<Manager>) -> Result<()> {
    match cmd {
        Commands::Start { target } => {
            // First try direct path
            if Path::new(&target).exists() {
                if let Err(e) = manager.start_from_config(target).await {
                    error!("start failed: {}", e);
                    println!("Error: {}", e);
                }
                return Ok(());
            }

            // Try directory with fpm.config.toml
            let dir = Path::new(&target);
            if dir.is_dir() {
                let candidate = dir.join("fpm.config.toml");
                if candidate.exists() {
                    if let Err(e) = manager.start_from_config(target).await {
                        error!("start failed: {}", e);
                        println!("Error: {}", e);
                    }
                    return Ok(());
                }
            }

            // Try existing process name
            {
                let spec = {
                    let specs = manager.specs.read().await;
                    specs.get(&target).cloned()
                };
                if let Some(spec) = spec {
                    if let Err(e) = manager.start_process(spec).await {
                        error!("start failed: {}", e);
                        println!("Error: {}", e);
                    }
                } else {
                    println!(
                        "Error: No process named '{}' and no config file found",
                        target
                    );
                }
            }
        }
        Commands::Stop { name } => {
            if let Err(e) = manager.stop_process(&name).await {
                error!("stop failed: {}", e);
            }
        }
        Commands::Restart { name } => {
            if let Err(e) = manager.stop_process(&name).await {
                error!("restart(stop) failed: {}", e);
            }
            let specs = manager.specs.read().await;
            if let Some(spec) = specs.get(&name) {
                if let Err(e) = manager.start_process(spec.clone()).await {
                    error!("restart(start) failed: {}", e);
                }
            } else {
                println!("no such process");
            }
        }
        Commands::List => manager.list().await,
        Commands::Status { name } => manager.status(name).await,
        Commands::Logs { name, lines } => {
            if let Err(e) = manager.tail_logs(&name, lines.unwrap_or(100)).await {
                error!("logs failed: {}", e);
            }
        }
        Commands::Daemon => {
            // Autostart all specs
            let specs = manager.specs.read().await;
            for spec in specs.values().cloned() {
                if spec.autostart {
                    if let Err(e) = manager.start_process(spec).await {
                        error!("autostart failed: {}", e);
                    }
                }
            }

            tracing::info!("daemon running; Ctrl+C to stop");
            tokio::signal::ctrl_c().await?;
            tracing::info!("daemon shutting down");
        }
    }

    Ok(())
}

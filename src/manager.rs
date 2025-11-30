use anyhow::{Context, Result};
use chrono::Local;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::process::Command;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct InfoConfig {
    name: String,
    autostart: Option<bool>,
    maxmemory: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CommandConfig {
    prerun: Option<String>,
    startup: String,
    after: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct TomlConfig {
    info: InfoConfig,
    commands: CommandConfig,
    run_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessSpec {
    pub name: String,
    pub prerun: Option<String>,
    pub startup: String,
    pub after: Option<String>,
    pub autostart: bool,
    pub maxmemory: Option<u64>,
    pub dir: String,
}

#[derive(Serialize, Deserialize, Default)]
struct AppStateFile {
    processes: Vec<ProcessSpec>,
}

#[derive(Debug, Clone)]
pub enum ProcessStatus {
    Running,
    Stopped,
    Finished,
}

pub struct ProcessInfo {
    pub stop_sender: tokio::sync::oneshot::Sender<()>,
    pub status: ProcessStatus,
}

pub struct Manager {
    state_file: PathBuf,
    pub specs: RwLock<HashMap<String, ProcessSpec>>,
    runtime_dir: PathBuf,
    pub processes: RwLock<HashMap<String, ProcessInfo>>,
}

impl Manager {
    pub async fn new() -> Result<Arc<Self>> {
        let proj = ProjectDirs::from("dev", "fpm", "fpm").context("no project dirs")?;
        let cfg = proj.config_dir();
        tokio::fs::create_dir_all(cfg).await?;
        let runtime_dir = cfg.join("runtime");
        tokio::fs::create_dir_all(&runtime_dir).await?;

        let m = Arc::new(Self {
            state_file: cfg.join("processes.json"),
            specs: RwLock::new(HashMap::new()),
            runtime_dir,
            processes: RwLock::new(HashMap::new()),
        });

        m.load().await?;
        Ok(m)
    }

    async fn load(self: &Arc<Self>) -> Result<()> {
        if !self.state_file.exists() {
            self.persist().await?;
            return Ok(());
        }
        let data = tokio::fs::read(&self.state_file).await?;
        let parsed: AppStateFile = serde_json::from_slice(&data)?;

        let mut specs = self.specs.write().await;
        specs.clear();
        for p in parsed.processes {
            specs.insert(p.name.clone(), p);
        }
        Ok(())
    }

    async fn persist(self: &Arc<Self>) -> Result<()> {
        let specs = self.specs.read().await;
        let state = AppStateFile {
            processes: specs.values().cloned().collect(),
        };
        let data = serde_json::to_vec_pretty(&state)?;
        tokio::fs::write(&self.state_file, data).await?;
        Ok(())
    }

    pub async fn start_from_config(self: &Arc<Self>, path: String) -> Result<()> {
        let p = Path::new(&path);
        let cfg_path = if p.is_dir() {
            let candidate = p.join("fpm.config.toml");
            if !candidate.exists() {
                anyhow::bail!("No fpm.config.toml found in {}", p.display());
            }
            candidate
        } else {
            if !p.exists() {
                anyhow::bail!("Config file not found: {}", p.display());
            }
            p.to_path_buf()
        };

        info!("Loading config from {}", cfg_path.display());
        let text = tokio::fs::read_to_string(&cfg_path).await?;
        let cfg: TomlConfig = toml::from_str(&text)?;

        let spec = ProcessSpec {
            name: cfg.info.name.clone(),
            prerun: cfg.commands.prerun.clone(),
            startup: cfg.commands.startup.clone(),
            after: cfg.commands.after.clone(),
            autostart: cfg.info.autostart.unwrap_or(false),
            maxmemory: cfg.info.maxmemory,
            dir: cfg.run_dir.unwrap_or_else(|| {
                cfg_path
                    .parent()
                    .unwrap_or(Path::new("."))
                    .to_string_lossy()
                    .to_string()
            }), // default to config file dir
        };

        self.start_process(spec).await
    }

    pub async fn start_process(self: &Arc<Self>, spec: ProcessSpec) -> Result<()> {
        let name = spec.name.clone();
        {
            let mut specs = self.specs.write().await;
            specs.insert(name.clone(), spec.clone());
        }
        self.persist().await?;

        self.stop_child(&name).await.ok();

        let (tx, mut rx) = tokio::sync::oneshot::channel();

        self.processes.write().await.insert(
            name.clone(),
            ProcessInfo {
                stop_sender: tx,
                status: ProcessStatus::Running,
            },
        );

        let log_path = self.runtime_dir.join(format!("{}.log", name));
        let spec_clone = spec.clone();
        let manager = Arc::clone(self);

        tokio::spawn(async move {
            loop {
                match rx.try_recv() {
                    Ok(_) | Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                        {
                            let mut processes = manager.processes.write().await;
                            if let Some(info) = processes.get_mut(&spec_clone.name) {
                                info.status = ProcessStatus::Stopped;
                            }
                        }
                        break;
                    }
                    Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {}
                }

                if let Err(e) = manager.run_process(&spec_clone, &log_path).await {
                    error!("Process {} run error: {}", spec_clone.name, e);
                }

                if !spec_clone.autostart {
                    manager
                        .processes
                        .write()
                        .await
                        .entry(spec_clone.name.clone())
                        .and_modify(|info| info.status = ProcessStatus::Finished);
                    break;
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }

            let mut processes = manager.processes.write().await;
            if let Some(info) = processes.get(&spec_clone.name) {
                if matches!(info.status, ProcessStatus::Stopped) {
                    processes.remove(&spec_clone.name);
                }
            }
            info!("Supervisor for {} exiting", spec_clone.name);
        });

        info!("Started supervisor for {}", name);
        Ok(())
    }

    async fn run_process(self: &Arc<Self>, spec: &ProcessSpec, log_path: &PathBuf) -> Result<()> {
        let mut log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        if let Some(pre) = &spec.prerun {
            let line = format!("[{}] PRERUN: {}\n", Local::now(), pre);
            log.write_all(line.as_bytes())?;
            log.flush()?;
            info!("[{}] prerun: {}", spec.name, pre);
            if let Err(e) = run_simple(pre, &spec.dir).await {
                error!("prerun failed for {}: {}", spec.name, e);
            }
        }

        let start_line = format!("[{}] START: {}\n", Local::now(), spec.startup);
        log.write_all(start_line.as_bytes())?;
        log.flush()?;
        info!("[{}] starting: {}", spec.name, spec.startup);

        let status = match run_simple(&spec.startup, &spec.dir).await {
            Ok(s) => s,
            Err(e) => {
                error!("startup failed for {}: {}", spec.name, e);
                let _ = writeln!(log, "[{}] STARTUP ERROR: {}", Local::now(), e);
                log.flush()?;
                return Err(e);
            }
        };

        let exit_line = format!("[{}] EXIT: {:?}\n", Local::now(), status.code());
        log.write_all(exit_line.as_bytes())?;

        if let Some(after) = &spec.after {
            let after_line = format!("[{}] AFTER: {}\n", Local::now(), after);
            log.write_all(after_line.as_bytes())?;
            log.flush()?;
            info!("[{}] running after: {}", spec.name, after);
            if let Err(e) = run_simple(after, &spec.dir).await {
                warn!("after hook failed for {}: {}", spec.name, e);
            }
        }

        log.flush()?;
        info!(
            "[{}] process finished with status {:?}",
            spec.name,
            status.code()
        );
        Ok(())
    }

    async fn stop_child(&self, name: &str) -> Result<()> {
        let mut processes = self.processes.write().await;
        if let Some(process_info) = processes.remove(name) {
            let _ = process_info.stop_sender.send(());
            info!("Sent stop signal to {}", name);
        }
        Ok(())
    }

    pub async fn stop_process(self: &Arc<Self>, name: &str) -> Result<()> {
        self.stop_child(name).await?;
        self.specs.write().await.remove(name);
        self.persist().await?;
        info!("Stopped and removed process {}", name);
        Ok(())
    }

    pub async fn list(&self) {
        let specs = self.specs.read().await;
        println!("{:<20} {:<10} {:<10}", "NAME", "RUNNING", "AUTOSTART");
        for (name, spec) in specs.iter() {
            let processes = self.processes.read().await;
            let running = matches!(
                processes.get(name).map(|p| &p.status),
                Some(ProcessStatus::Running)
            );
            println!("{:<20} {:<10} {:<10}", name, running, spec.autostart);
        }
    }

    pub async fn status(&self, name: Option<String>) {
        let specs = self.specs.read().await;
        if let Some(n) = name {
            if let Some(spec) = specs.get(&n) {
                let processes = self.processes.read().await;
                let status_str = match processes.get(&n).map(|p| &p.status) {
                    Some(ProcessStatus::Running) => "running",
                    Some(ProcessStatus::Stopped) => "stopped",
                    Some(ProcessStatus::Finished) => "finished",
                    None => "not started",
                };
                println!("{}: {} (startup: {})", n, status_str, spec.startup);
            } else {
                println!("no such process");
            }
        } else {
            drop(specs);
            self.list().await;
        }
    }

    pub async fn tail_logs(&self, name: &str, lines: usize) -> Result<()> {
        let path = self.runtime_dir.join(format!("{}.log", name));
        if !path.exists() {
            println!("no logs");
            return Ok(());
        }
        let data = tokio::fs::read_to_string(path).await?;
        let l: Vec<&str> = data.lines().collect();
        let start = l.len().saturating_sub(lines);
        for line in &l[start..] {
            println!("{}", line);
        }
        Ok(())
    }
}

async fn run_simple(cmd: &str, dir: &str) -> Result<std::process::ExitStatus> {
    // we'll use `sh -c` to preserve behavior like environment variable expansion and pipes (Subject to change)
    #[cfg(target_family = "unix")]
    {
        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(dir)
            .spawn()?
            .wait()
            .await?;
        Ok(status)
    }

    #[cfg(not(target_family = "unix"))]
    {
        // fallback naive split for non-unix
        let mut parts = cmd.split_whitespace();
        let bin = parts.next().context("empty command")?.to_string();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();

        let status = Command::new(bin)
            .args(args)
            .current_dir(dir)
            .spawn()?
            .wait()
            .await?;

        Ok(status)
    }
}

use crate::{
    commands::{RunMode, WatchOptions},
    utils::*,
};

use std::process::Stdio;
use tokio::process::Command;

pub struct RunAppExecutor {
    mode: RunMode,
}

impl RunAppExecutor {
    pub async fn execute(mode: Option<RunMode>) -> anyhow::Result<()> {
        let mode = mode.unwrap_or(RunMode::Normal);
        let executor = Self { mode };

        match &executor.mode {
            RunMode::Normal => executor.run_normal_mode().await,
            RunMode::Watch { options } => executor.run_watch_mode(&options).await,
        }
    }

    async fn run_watch_mode(&self, options: &WatchOptions) -> anyhow::Result<()> {
        if !is_watchexec_installed() {
            watchexec_installation_prompt().await?;
        }

        let mut args = vec!["--restart".to_string()];

        if options.clear {
            args.push("--clear".to_string());
        }

        for dir in options.watch.split(',') {
            args.push("--watch".to_string());
            args.push(dir.trim().to_string());
        }

        args.push("--exts".to_string());
        args.push("rs,toml".to_string());

        let default_ignores = vec![
            "target/**",
            "node_modules/**",
            ".git/**",
            "dist/**",
            "*.tmp",
            "*.log",
        ];

        for pattern in &default_ignores {
            args.push("--ignore".to_string());
            args.push(pattern.to_string());
        }

        if let Some(user_ignores) = &options.ignore {
            for pattern in user_ignores.split(',') {
                let pattern = pattern.trim();
                if !pattern.is_empty() {
                    args.push("--ignore".to_string());
                    args.push(pattern.to_string());
                }
            }
        }

        args.push("--".to_string());
        args.push("cargo".to_string());
        args.push("run".to_string());

        if let Some(bin_name) = &options.bin {
            args.push("--bin".to_string());
            args.push(bin_name.clone());
        }

        let mut child = Command::new("watchexec")
            .args(&args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()?;

        let status = child.wait().await?;

        if !status.success() {
            println!("watchexec exited with status: {}", status);
            std::process::exit(status.code().unwrap_or(1));
        }

        Ok(())
    }

    async fn run_normal_mode(&self) -> anyhow::Result<()> {
        let mut child = Command::new("cargo")
            .arg("run")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()?;

        let status = child.wait().await?;

        if !status.success() {
            println!("cargo run exited with status: {}", status);
            std::process::exit(status.code().unwrap_or(1));
        }

        Ok(())
    }
}

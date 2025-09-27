use crate::commands::CreateAppInput;

use std::process::Stdio;
use tokio::process::Command;
use which::which;

pub struct CreateAppExecutor;

impl CreateAppExecutor {
    pub async fn execute(input: CreateAppInput) -> anyhow::Result<()> {
        println!("🚀Creating application: {}", input.name);

        match input.template.as_str() {
            "None" => Self::run_empty_project_setup(&input.name).await?,
            _ => Self::run_template_setup(&input.template).await?,
        }

        if input.init_git {
            Self::run_git_initialization(&input.name).await?;
        }

        if input.has_extra_libs() {
            Self::run_install_extra_libs(&input).await?;
        }

        println!("\n✅ Application '{}' created successfully!\n", input.name);

        Ok(())
    }

    async fn run_empty_project_setup(project_name: &str) -> anyhow::Result<()> {
        let mut child = Command::new("cargo")
            .arg("new")
            .arg(project_name)
            .stdout(Stdio::inherit())
            .spawn()?;

        let status = child.wait().await?;

        if !status.success() {
            println!("❌cargo new exited with status: {}", status);
            std::process::exit(status.code().unwrap_or(1));
        }

        Ok(())
    }

    /// TODO: Implement template setup logic here
    /// - Download template repo as zip
    /// - Unzip it to the project directory
    /// - Remove unnecessary files
    /// - Customize cargo toml crate name
    async fn run_template_setup(_: &str) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_git_initialization(project_name: &str) -> anyhow::Result<()> {
        if which("git").is_err() {
            println!("❌git is not installed or not found in PATH.");
            return Ok(());
        }

        let mut child = Command::new("git")
            .arg("init")
            .current_dir(project_name)
            .stdout(Stdio::null())
            .spawn()?;

        let status = child.wait().await?;

        if !status.success() {
            println!("❌git init exited with status: {}", status);
            std::process::exit(status.code().unwrap_or(1));
        }

        Ok(())
    }

    async fn run_install_extra_libs(input: &CreateAppInput) -> anyhow::Result<()> {
        let libs = input
            .extra_libs
            .iter()
            .map(|s| s.split(" ").next().unwrap_or(""))
            .collect::<Vec<&str>>()
            .join(" ");

        let mut child = Command::new("cargo")
            .arg("add")
            .args(libs.split(' '))
            .current_dir(&input.name)
            .stdout(Stdio::inherit())
            .spawn()?;

        let status = child.wait().await?;

        if !status.success() {
            println!("❌cargo add exited with status: {}", status);
            std::process::exit(status.code().unwrap_or(1));
        }

        Ok(())
    }
}

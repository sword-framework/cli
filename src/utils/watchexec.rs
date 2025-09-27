use inquire::Confirm;
use std::process::Stdio;
use tokio::process::Command;
use which::which;

pub fn is_watchexec_installed() -> bool {
    which("watchexec").is_ok()
}

pub async fn watchexec_installation_prompt() -> anyhow::Result<()> {
    println!("❌watchexec-cli is not installed.");

    let install = Confirm::new("Do you want to install watchexec-cli now?")
        .with_default(true)
        .prompt()?;

    println!();

    if !install {
        println!("❌watchexec-cli is required for watch mode. Exiting...\n");
        std::process::exit(1);
    }

    install_watchexec().await?;

    Ok(())
}

async fn install_watchexec() -> anyhow::Result<()> {
    println!("📦Installing watchexec-cli with cargo...");

    let mut install_child = Command::new("cargo")
        .args(&["install", "watchexec-cli"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let install_status = install_child.wait().await?;

    if !install_status.success() {
        println!("❌Failed to install watchexec-cli.");
        std::process::exit(1);
    }

    println!("✅watchexec-cli installed successfully.");

    Ok(())
}

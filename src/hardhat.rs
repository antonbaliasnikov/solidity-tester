use std::path::Path;
use std::process::Command;

use crate::cast::Wallet;
use crate::constants;
use crate::io_ext;
use crate::project::Project;

/// Set default environment variables for Hardhat commands.
fn set_default_env(cmd: &mut Command, wallet: &Wallet) {
    cmd.env("NETWORK_NAME", constants::NETWORK_NAME);
    cmd.env("RPC_URL", constants::RPC_URL);
    cmd.env("CHAIN_ID", constants::CHAIN_ID);
    cmd.env("PRIVATE_KEY", &wallet.private_key);
}

/// Build a `hardhat` command with shared flags/envs.
pub fn cmd(command: &str, project: &Project, wallet: &Wallet) -> Command {
    let mut cmd = Command::new("npm");
    cmd.arg("run");
    cmd.arg(command);

    // Set default environment variables
    set_default_env(&mut cmd, wallet);

    // Set custom environment variables if provided
    if let Some(envs) = &project.env {
        cmd.envs(envs);
    }

    // Set the current directory to the repository path
    let repo_dir = Path::new(constants::DEFAULT_REPOS_DIR).join(&project.name);
    cmd.current_dir(&repo_dir);
    cmd
}

/// Install dependencies for a Hardhat project.
pub fn install(project: &Project) -> anyhow::Result<()> {
    // We use io_ext cmd here to respect the package manager (npm/yarn)
    let mut hh_install = io_ext::cmd("install", project);
    hh_install.arg("--force");
    let install_status = io_ext::run(hh_install)?;
    if !install_status.success() {
        anyhow::bail!("Project {} installation failed!", &project.name);
    }
    Ok(())
}

/// Clean a Hardhat project.
pub fn clean(project: &Project, wallet: &Wallet) -> anyhow::Result<()> {
    let hh_clean = cmd("clean", project, wallet);
    let clean_status = io_ext::run(hh_clean)?;
    if !clean_status.success() {
        anyhow::bail!("Project {} cleaning failed!", &project.name);
    }
    Ok(())
}

/// Compile a Hardhat project.
pub fn compile(project: &Project, wallet: &Wallet) -> anyhow::Result<()> {
    let hh_compile = cmd("compile", project, wallet);
    let compile_status = io_ext::run(hh_compile)?;
    if !compile_status.success() {
        anyhow::bail!("Project {} compilation failed!", &project.name);
    }
    Ok(())
}

/// Deploy a Hardhat project.
pub fn deploy(project: &Project, wallet: &Wallet) -> anyhow::Result<()> {
    // Clean up previous deployments if any
    let deployments_dir = Path::new(constants::DEFAULT_REPOS_DIR)
        .join(&project.name)
        .join("deployments");
    io_ext::delete_dir(&deployments_dir)?;

    let hh_deploy = cmd("deploy", project, wallet);
    let deploy_status = io_ext::run(hh_deploy)?;
    if !deploy_status.success() {
        anyhow::bail!("Project {} deployment failed!", &project.name);
    }
    Ok(())
}

/// Run tests for a Hardhat project.
pub fn test(project: &Project, wallet: &Wallet) -> anyhow::Result<()> {
    let hh_test = cmd("test", project, wallet);
    let run_status = io_ext::run(hh_test)?;
    if !run_status.success() {
        anyhow::bail!("Project {} tests failed!", &project.name);
    }
    Ok(())
}

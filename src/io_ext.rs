//! I/O utilities

use crate::constants;
use crate::project::Project;
use anyhow::Context;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};
use which::which;

/// Check if an external command is available in PATH.
fn have(cmd: &str) -> bool {
    which(cmd).is_ok()
}

/// Check that required external commands are available in PATH.
pub fn check_prerequisites() -> anyhow::Result<()> {
    if !have("git") {
        anyhow::bail!("git not found in PATH");
    }
    if !have("forge") {
        anyhow::bail!("forge not found in PATH");
    }
    Ok(())
}

/// Ensure a directory exists (like `mkdir -p`).
pub fn ensure_dir(p: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(p).with_context(|| format!("mkdir -p {}", p.display()))
}

/// Remove directory recursively if it exists.
pub fn delete_dir(p: &Path) -> anyhow::Result<()> {
    if p.exists() {
        fs::remove_dir_all(p).with_context(|| format!("Remove directory {}", p.display()))?;
    }
    Ok(())
}

/// Build a command for project.
pub fn cmd(command: &str, project: &Project) -> Command {
    let project_manager = project
        .package_manager
        .clone()
        .unwrap_or(constants::DEFAULT_PROJECT_MANAGER.to_string());
    let mut cmd = Command::new(project_manager);
    cmd.arg(command);

    // Set custom environment variables if provided
    if let Some(envs) = &project.env {
        cmd.envs(envs);
    }

    // Set the current directory to the repository path
    let repo_dir = Path::new(constants::DEFAULT_REPOS_DIR).join(&project.name);
    cmd.current_dir(&repo_dir);
    cmd
}

/// Run a command, capturing exit code, stdout, stderr.
pub fn run(mut cmd: Command) -> anyhow::Result<std::process::ExitStatus> {
    let out = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn {:#?}", &cmd))?
        .wait_with_output()
        .with_context(|| format!("wait {:#?}", &cmd))?;

    // Print to console to be caught by JUnit reporter sections
    let stdout_str = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr_str = String::from_utf8_lossy(&out.stderr).into_owned();
    println!("{stdout_str}");
    eprintln!("{stderr_str}");

    Ok(out.status)
}

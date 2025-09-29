//! Forge-specific helpers: pragma pinning, foundry.toml normalization,
//! running `forge build`, `forge build --sizes`, `forge test`, wrapping outputs.

use std::collections::BTreeMap;
use std::{path::Path, process::Command};

/// Internal: build a `forge` command with shared flags/envs.
#[allow(dead_code)]
fn forge_cmd(
    command: &str,
    repo: &Path,
    project_env: Option<&BTreeMap<String, String>>,
) -> Command {
    let mut cmd = Command::new("forge");
    cmd.arg(command);
    if let Some(envmap) = project_env {
        cmd.envs(envmap);
    }
    cmd.current_dir(repo);
    cmd
}

//! Git operations: persistent cloning + forced reset before each run.

use crate::io_ext::{ensure_dir, run};
use anyhow::Result;
use std::path::Path;

/// Clone repo if missing (depth=1 + submodules).
pub fn clone(dest_dir: &Path, repo_url: &str) -> Result<()> {
    if !dest_dir.exists() {
        ensure_dir(dest_dir.parent().unwrap())?;
        let mut cmd = std::process::Command::new("git");
        cmd.args([
            "clone",
            "--depth",
            "1",
            "--recurse-submodules",
            repo_url,
            dest_dir.to_str().unwrap(),
        ]);
        run(cmd)?;
    }
    Ok(())
}

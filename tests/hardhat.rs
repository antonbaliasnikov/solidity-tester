//! rstest driver: one test per project (file-based).
//! For each project it runs the compiler matrix across via-ir modes,
//! with persistent clones and hard reset between runs.

use rstest::rstest;
use solidity_tester::constants;
use solidity_tester::hardhat;
use solidity_tester::{git, io_ext, project::Project};
use std::path::Path;
use std::path::PathBuf;

#[rstest]
fn hardhat_test(#[files("tests/_gen/hardhat/*.toml")] path: PathBuf) -> anyhow::Result<()> {
    // Check prerequisites
    io_ext::check_prerequisites()?;

    // Load project from a test file
    let project = Project::from_file(&path)?;

    // Check if the project is disabled
    if project.disabled.unwrap_or(false) {
        eprintln!("Project {} disabled. Skipping...", project.name);
        return Ok(());
    }

    // Remove any existing repository
    let repo_dir = Path::new(constants::DEFAULT_REPOS_DIR).join(&project.name);
    io_ext::delete_dir(&repo_dir)?;

    // Clone repository
    git::clone(&repo_dir, &project.repo)?;

    // Install required JS deps if any
    hardhat::install(&project)?;

    // Clean-up project (if needed)
    hardhat::clean(&project)?;

    // Compile hardhat project
    hardhat::compile(&project)?;

    // Deploy hardhat project
    hardhat::deploy(&project)?;

    // Run hardhat tests
    hardhat::test(&project)?;

    // Clean-up repository
    io_ext::delete_dir(&repo_dir)?;

    Ok(())
}

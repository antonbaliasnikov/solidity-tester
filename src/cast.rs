use crate::constants;
use crate::io_ext;
use anyhow::Context;
use fs2::FileExt;
use serde::Deserialize;
use std::io::Write;
use std::process::Command;
use std::{fs::OpenOptions, path::PathBuf};

#[derive(Clone, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub private_key: String,
}

/// Set default environment variables for cast commands.
fn set_default_env(cmd: &mut Command) {
    cmd.env("RPC_URL", constants::RPC_URL);
    cmd.env("PRIVATE_KEY", constants::RICH_ACCOUNT_PRIVATE_KEY);
}

/// Build a `cast` command with shared flags/envs.
pub fn cmd(command: &str) -> Command {
    let mut cmd = Command::new("cast");
    cmd.arg(command);
    // Set default environment variables
    set_default_env(&mut cmd);
    cmd
}

/// Create a new wallet account using `cast wallet new`.
pub fn create_account() -> anyhow::Result<Wallet> {
    let mut cast_cmd = cmd("wallet");
    cast_cmd.arg("new");
    cast_cmd.arg("--json");
    let output = cast_cmd
        .output()
        .with_context(|| "Creating a new wallet.")?;

    if !output.status.success() {
        anyhow::bail!("Creating new wallet failed!");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let wallets: Vec<Wallet> =
        serde_json::from_str(&stdout).context("Failed to parse `cast` JSON output!")?;
    let wallet = wallets
        .first()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("No wallet entry found!"))?;

    Ok(wallet)
}

/// Create a stable lock-file path per (funder address).
fn funding_lock_path(funder_addr: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    // one lock per funding account; adjust if you want one per rpc too
    p.push(format!("funding-lock-{funder_addr}.lock"));
    p
}

/// Fund an account with specified amount of ETH using `cast send`.
pub fn fund_account(address: &str, amount_eth: f64) -> anyhow::Result<()> {
    // ---- acquire process-wide lock (blocks until available) ----
    let funder_addr = constants::RICH_ACCOUNT_PRIVATE_KEY; // derive from your private key once
    let lock_path = funding_lock_path(funder_addr);
    let lock_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&lock_path)
        .with_context(|| format!("open lock file at {lock_path:?}"))?;

    // Optional: mark file so it's obvious what it is
    // (ignore errors; not required)
    let _ = lock_file.set_len(0).and_then(|_| {
        let mut f = &lock_file;
        f.write_all(b"funding lock; safe to delete")
    });

    lock_file
        .lock_exclusive()
        .with_context(|| format!("acquire funding lock at {lock_path:?}"))?;

    // The lock is held until `lock_file` is dropped (or you call unlock()).

    // ---- critical section: do the nonce-sensitive send ----
    let result = (|| -> anyhow::Result<()> {
        let mut cast_cmd = cmd("send");
        cast_cmd.arg(address);
        cast_cmd.arg(format!("--value={amount_eth}ether"));
        cast_cmd.arg(format!(
            "--private-key={}",
            constants::RICH_ACCOUNT_PRIVATE_KEY
        ));
        cast_cmd.arg(format!("--rpc-url={}", constants::RPC_URL));

        let output = io_ext::run(cast_cmd)?;
        if !output.success() {
            anyhow::bail!("Funding account {address} failed!");
        }
        Ok(())
    })();

    fs2::FileExt::unlock(&lock_file)?;

    result
}

/// Check that an address balance is not zero using `cast balance`.
pub fn check_balance(address: &str) -> anyhow::Result<u128> {
    let mut cast_cmd = cmd("balance");
    cast_cmd.arg(address);
    cast_cmd.arg(format!("--rpc-url={}", constants::RPC_URL));

    let output = cast_cmd.output().context("Failed to run `cast balance`!")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("`cast balance` failed: {}", stderr.trim());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let balance_str = stdout.trim();

    // Try to parse as decimal number (wei or ether — cast usually prints wei by default)
    let balance: u128 = balance_str
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow::anyhow!("unexpected `cast balance` output: {balance_str}"))?
        .parse()
        .context("Failed to parse balance!")?;

    if balance == 0 {
        anyhow::bail!("Account balance is zero!");
    }

    Ok(balance)
}

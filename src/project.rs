//! Project metadata + loader used in tests and reports.

use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub repo: String,
    pub toolchain: String,
    pub description: Option<String>,
    pub disabled: Option<bool>,
    pub requires_js_setup: Option<bool>,
    pub package_manager: Option<String>,
    pub compile_only: Option<bool>,
    pub run_iterations: Option<u32>,
    pub env: Option<BTreeMap<String, String>>,
}

impl Project {
    /// Load a flattened per-project TOML file (from `tests/_gen`).
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        Ok(toml::from_str::<Self>(&fs::read_to_string(path)?)?)
    }
}

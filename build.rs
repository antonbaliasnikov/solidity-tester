use serde::Deserialize;
use std::collections::BTreeMap;
use std::{fs, io::Write, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=tests/projects.toml");
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

    let src = fs::read_to_string(PathBuf::from(std::env::var("PROJECTS_TOML").unwrap()))?;
    let table: toml::value::Table = toml::from_str(&src)?;

    let gen_dir = root.join("tests/_gen");
    fs::create_dir_all(&gen_dir)?;

    for (name, val) in &table {
        let p: Project = val.clone().try_into()?;
        let final_dir = gen_dir.join(&p.toolchain);
        fs::create_dir_all(&final_dir)?;
        let mut f = fs::File::create(final_dir.join(format!("{name}.toml")))?;
        writeln!(f, r#"name = "{name}""#)?;
        writeln!(f, r#"repo = "{}""#, p.repo.replace('"', "\\\""))?;
        writeln!(f, r#"toolchain = "{}""#, p.toolchain.replace('"', "\\\""))?;
        if let Some(d) = &p.description {
            writeln!(f, r#"description = "{}""#, d.replace('"', "\\\""))?;
        }
        if let Some(b) = p.disabled {
            writeln!(f, "disabled = {b}")?;
        }
        if let Some(b) = p.requires_js_setup {
            writeln!(f, "requires_js_setup = {b}")?;
        }
        if let Some(pm) = &p.package_manager {
            writeln!(f, r#"package_manager = "{}""#, pm.replace('"', "\\\""))?;
        }
        if let Some(b) = p.compile_only {
            writeln!(f, "compile_only = {b}")?;
        }
        if let Some(n) = p.run_iterations {
            writeln!(f, "run_iterations = {n}")?;
        }
        if let Some(env) = p.env {
            writeln!(f, "[env]")?;
            for (k, v) in env {
                writeln!(f, "{k} = \"{v}\"")?;
            }
        }
    }
    Ok(())
}

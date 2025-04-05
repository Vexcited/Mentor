use crate::utils::{open_file, read_file, write_file};
use anyhow::Result;
use std::{fs::File, io, process::Command};

pub const CARGO_TOML: &str = "Cargo.toml";

pub fn open_cargo_toml() -> io::Result<File> {
  open_file(CARGO_TOML)
}

pub fn get_current_version() -> Result<String> {
  let content = read_file(&mut open_cargo_toml()?)?;
  let content: toml::Value = toml::from_str(&content)?;

  let version = content
    .get("package")
    .and_then(|package| package.get("version"))
    .and_then(|version| version.as_str())
    .ok_or_else(|| anyhow::anyhow!("'Cargo.toml' is missing 'version' property."))?;

  Ok(version.to_string())
}

/// Edits the `Cargo.toml` file and updates the value of the `version` property.
pub fn bump_version(version: &str) -> Result<()> {
  let mut file = open_cargo_toml()?;

  let content = read_file(&mut file)?;
  let mut content: toml::Value = toml::from_str(&content)?;

  let version_property = content
    .get_mut("package")
    .and_then(|package| package.get_mut("version"))
    .ok_or_else(|| anyhow::anyhow!("'Cargo.toml' is missing 'version' property."))?;

  *version_property = toml::Value::String(version.to_string());

  let content = toml::to_string(&content)?;
  write_file(&mut file, content)?;

  // We have to update the `Cargo.lock` file as well.
  if !Command::new("cargo")
    .arg("check")
    .output()?
    .status
    .success()
  {
    return Err(anyhow::anyhow!("Failed to update 'Cargo.lock' file."));
  }

  Ok(())
}

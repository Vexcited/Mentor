use crate::utils::read_file;
use anyhow::Result;
use std::{
  fs::File,
  io::{self, Write},
};

pub fn open_cargo_toml() -> io::Result<File> {
  File::open("Cargo.toml")
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

  file.set_len(0)?;
  file.write_all(content.to_string().as_bytes())?;
  file.flush()?;

  Ok(())
}

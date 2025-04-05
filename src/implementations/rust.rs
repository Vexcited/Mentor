use crate::utils::read_file;
use anyhow::Result;
use std::{fs::File, io};

pub fn open_cargo_toml() -> io::Result<File> {
  File::open("Cargo.toml")
}

pub fn get_current_version() -> Result<String> {
  let content = read_file(&mut open_cargo_toml()?)?;

  // Find the `version = "x.y.z"` line.
  let version_line = content
    .lines()
    .find(|line| line.contains("version"))
    .ok_or_else(|| anyhow::anyhow!("'Cargo.toml' is missing 'version' property."))?;

  let version = version_line
    .split("\"")
    .nth(1)
    .ok_or_else(|| anyhow::anyhow!("'Cargo.toml' is missing 'version' property."))?;

  Ok(version.to_string())
}

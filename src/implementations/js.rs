use anyhow::{Ok, Result};
use std::{
  fs::File,
  io::{self, Write},
  process::Command,
};

use crate::utils::read_file;

pub fn run_checks() -> Result<()> {
  // We're checking the code style of the project.
  let output = Command::new("bun").arg("eslint").output()?;

  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stdout);
    return Err(anyhow::anyhow!(
      "failed to check codestyle, see the following stack trace:\n\n{error}"
    ));
  }

  // We're checking the types of the project.
  let output = Command::new("bun").arg("tsc").arg("--noEmit").output()?;

  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stdout);
    return Err(anyhow::anyhow!(
      "failed to check types, see the following stack trace:\n\n{error}"
    ));
  }

  // We're checking the tests of the project.
  let output = Command::new("bun").arg("test").output()?;

  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stdout);
    return Err(anyhow::anyhow!(
      "failed to pass tests, see the following stack trace:\n\n{error}"
    ));
  }

  Ok(())
}

pub fn open_package_json() -> io::Result<File> {
  File::open("package.json")
}

/// Reads the `package.json` file and parses it as JSON
/// and returns the value of the `version` property as string.
pub fn get_current_version() -> Result<String> {
  let file = open_package_json()?;
  let json: serde_json::Value = serde_json::from_reader(file)?;

  let version = json
    .get("version")
    .ok_or_else(|| anyhow::anyhow!("'package.json' is missing 'version' property."))?
    .as_str()
    .ok_or_else(|| anyhow::anyhow!("'version' should be a string."))?;

  Ok(version.to_string())
}

/// Edits the `package.json` file and updates the value of the `version` property.
pub fn bump_version(version: &str) -> Result<()> {
  let mut file = open_package_json()?;

  let content = read_file(&mut file)?;
  let mut content: serde_json::Value = serde_json::from_str(&content)?;

  let version_property = content
    .get_mut("version")
    .ok_or_else(|| anyhow::anyhow!("'package.json' is missing 'version' property."))?;

  *version_property = serde_json::Value::String(version.to_string());

  file.set_len(0)?;
  file.write_all(content.to_string().as_bytes())?;
  file.flush()?;

  Ok(())
}

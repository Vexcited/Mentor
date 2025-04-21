use crate::utils::{open_file, read_file, write_file};
use anyhow::{Ok, Result};
use std::{fs::File, io, path::Path, process::Command};

fn detect_package_manager() -> Result<String> {
  // As of right now, we only support bun and pnpm.
  // Adding support for yarn and npm is not a priority right now...
  let lockfiles = [("bun.lockb", "bun"), ("pnpm-lock.yaml", "pnpm")];

  for (lockfile, package_manager) in lockfiles.iter() {
    if Path::new(lockfile).exists() {
      return Ok(package_manager.to_string());
    }
  }

  Err(anyhow::anyhow!("no lockfile found"))
}

pub fn run_checks() -> Result<()> {
  let package_manager = detect_package_manager()?;

  // We're checking the code style of the project.
  let output = Command::new(&package_manager).arg("eslint").output()?;

  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stdout);
    return Err(anyhow::anyhow!(
      "failed to check codestyle, see the following stack trace:\n\n{error}"
    ));
  }

  // We're checking the types of the project.
  let output = Command::new(&package_manager)
    .arg("tsc")
    .arg("--noEmit")
    .output()?;

  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stdout);
    return Err(anyhow::anyhow!(
      "failed to check types, see the following stack trace:\n\n{error}"
    ));
  }

  // We check the tests of a project, only if the package manager is bun
  // since it has a built-in test runner.
  if package_manager == "bun" {
    // We're checking the tests of the project.
    let output = Command::new(&package_manager).arg("test").output()?;

    if !output.status.success() {
      let error = String::from_utf8_lossy(&output.stdout);
      return Err(anyhow::anyhow!(
        "failed to pass tests, see the following stack trace:\n\n{error}"
      ));
    }
  }

  Ok(())
}

pub const PACKAGE_JSON: &str = "package.json";

pub fn open_package_json() -> io::Result<File> {
  open_file(PACKAGE_JSON)
}

/// Reads the `package.json` file and parses it as JSON
/// and returns the value of the `version` property as string.
pub fn get_current_version() -> Result<String> {
  let file = open_package_json()?;
  let json: serde_json::Value = serde_json::from_reader(file)?;

  let version = json
    .get("version")
    .ok_or_else(|| anyhow::anyhow!("'package.json' is missing 'version' property"))?
    .as_str()
    .ok_or_else(|| anyhow::anyhow!("'version' should be a string"))?;

  Ok(version.to_string())
}

/// Edits the `package.json` file and updates the value of the `version` property.
pub fn bump_version(version: &str) -> Result<()> {
  let mut file = open_package_json()?;

  let content = read_file(&mut file)?;
  let mut content: serde_json::Value = serde_json::from_str(&content)?;

  let version_property = content
    .get_mut("version")
    .ok_or_else(|| anyhow::anyhow!("'package.json' is missing 'version' property"))?;

  *version_property = serde_json::Value::String(version.to_string());

  let content = serde_json::to_string_pretty(&content)?;
  write_file(&mut file, content)?;

  Ok(())
}

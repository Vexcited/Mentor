use crate::utils::{open_file, read_file, write_file};
use anyhow::{Ok, Result};
use regex::Regex;
use std::{
  env::current_dir,
  fs::{read_dir, File},
  io,
  path::Path,
  process::Command,
};

fn detect_package_manager() -> Result<String> {
  // As of right now, we only support bun and pnpm.
  // Adding support for yarn and npm is not a priority right now...
  let lockfiles = [
    ("bun.lock", "bun"),
    ("bun.lockb", "bun"),
    ("pnpm-lock.yaml", "pnpm"),
  ];

  for (lockfile, package_manager) in lockfiles.iter() {
    if Path::new(lockfile).exists() {
      let package_manager = package_manager.to_string();

      #[cfg(windows)]
      let package_manager = format!(
        "{package_manager}{}",
        if package_manager == "pnpm" {
          ".cmd"
        } else {
          ""
        }
      );

      return Ok(package_manager);
    }
  }

  Err(anyhow::anyhow!("no lockfile found"))
}

/// Looking up for tests files according to Bun defaults.
/// https://bun.com/docs/cli/test#run-tests
fn has_test_files(dir: &Path) -> Result<bool> {
  let test_patterns = vec![
    Regex::new(r".*\.test\.(js|jsx|ts|tsx)$")?,
    Regex::new(r".*_test\.(js|jsx|ts|tsx)$")?,
    Regex::new(r".*\.spec\.(js|jsx|ts|tsx)$")?,
    Regex::new(r".*_spec\.(js|jsx|ts|tsx)$")?,
  ];

  fn check_directory(dir: &Path, patterns: &[Regex]) -> Result<bool> {
    let entries = read_dir(dir)?;

    for entry in entries {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() {
        if check_directory(&path, patterns)? {
          return Ok(true);
        }
      } else if path.is_file() {
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
          for pattern in patterns {
            if pattern.is_match(filename) {
              return Ok(true);
            }
          }
        }
      }
    }

    Ok(false)
  }

  check_directory(dir, &test_patterns)
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
    let current_dir = current_dir()?;

    if has_test_files(&current_dir)? {
      // We're checking the tests of the project.
      let output = Command::new(&package_manager).arg("test").output()?;

      if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow::anyhow!(
          "failed to pass tests, see the following stack trace:\n\n{error}"
        ));
      }
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

  // Add a newline at the end of the file, to avoid issue with eslint !
  let content = content + "\n";

  write_file(&mut file, content)?;

  Ok(())
}

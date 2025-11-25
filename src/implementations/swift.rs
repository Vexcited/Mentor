use crate::utils::{open_readme, read_file, write_file};
use crate::{git, version::DEFAULT};
use anyhow::Result;

pub const PACKAGE_SWIFT: &str = "Package.swift";

pub fn get_current_version() -> String {
  let tags = git::tags();

  if let Some(latest) = tags.first() {
    latest.to_string()
  }
  else {
    DEFAULT.into()
  }
}

fn bump_readme(old_version: &str, new_version: &str) -> Result<()> {
  let mut file = open_readme()?;
  let content = read_file(&mut file)?;

  // replace for installation section
  let from = format!("from: \"{old_version}\"");
  let to = format!("from: \"{new_version}\"");
  let content = content.replace(&from, &to);

  write_file(&mut file, content)?;

  Ok(())
}

pub fn bump_version(new_version: &str) -> Result<()> {
  let old_version = get_current_version();

  bump_readme(&old_version, new_version)?;

  Ok(())
}

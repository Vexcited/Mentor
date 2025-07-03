use crate::utils::{find_between, open_file, open_readme, read_file, write_file};
use anyhow::Result;
use std::{fs::File, io};

pub const BUILD_GRADLE_KTS: &str = "library/build.gradle.kts";

pub fn open_build_gradle_kts() -> io::Result<File> {
  open_file(BUILD_GRADLE_KTS)
}

/// Reads the `library/build.gradle.kts` file and parses it as KTS
/// and returns the value of the `version` property as string.
pub fn get_current_version() -> Result<String> {
  let content = read_file(&mut open_build_gradle_kts()?)?;

  // Find the `version = "x.y.z"` line.
  let version_line = content
    .lines()
    .find(|line| line.contains("version = \""))
    .ok_or_else(|| anyhow::anyhow!("'build.gradle.kts' is missing 'version' variable."))?;

  let version = version_line
    .split("\"")
    .nth(1)
    .ok_or_else(|| anyhow::anyhow!("'build.gradle.kts' is missing 'version' variable."))?;

  Ok(version.to_string())
}

fn bump_build_gradle_kts(old_version: &str, new_version: &str) -> Result<()> {
  let mut file = open_build_gradle_kts()?;
  let content = read_file(&mut file)?;

  let from = format!("version = \"{old_version}\"");
  let to = format!("version = \"{new_version}\"");

  // Replace the first occurrence of the version, since should be
  // located at the very first lines of the file.
  let content = content.replacen(&from, &to, 1);

  write_file(&mut file, content)?;

  Ok(())
}

fn bump_readme(old_version: &str, new_version: &str) -> Result<()> {
  let mut file = open_readme()?;
  let content = read_file(&mut file)?;
  let artifact_id = find_between(&content, "<artifactId>", "</artifactId>");

  // replace for maven section
  let from = format!("<version>{old_version}</version>");
  let to = format!("<version>{new_version}</version>");
  let content = content.replace(&from, &to);

  // replace for gradle (kotlin) section
  let from = format!("implementation(\"ink.literate:{artifact_id}:{old_version}\")");
  let to = format!("implementation(\"ink.literate:{artifact_id}:{new_version}\")");
  let content = content.replace(&from, &to);

  // replace for gradle section
  let from = format!("implementation 'ink.literate:{artifact_id}:{old_version}'");
  let to = format!("implementation 'ink.literate:{artifact_id}:{new_version}'");
  let content = content.replace(&from, &to);

  write_file(&mut file, content)?;

  Ok(())
}

pub fn bump_version(new_version: &str) -> Result<()> {
  let old_version = get_current_version()?;

  bump_build_gradle_kts(&old_version, new_version)?;
  bump_readme(&old_version, new_version)?;

  Ok(())
}

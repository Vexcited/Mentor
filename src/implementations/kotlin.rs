use crate::utils::find_between;
use std::{
  fs::File,
  io::{Read, Write},
};

pub fn run_checks() {
  // We don't have any checks to run for Kotlin.
}

fn read_build_gradle_kts() -> String {
  let mut file = File::open("library/build.gradle.kts").unwrap();

  let mut buffer = String::new();
  file.read_to_string(&mut buffer).unwrap();

  buffer
}

fn read_readme() -> String {
  let mut file = File::open("README.md").unwrap();

  let mut buffer = String::new();
  file.read_to_string(&mut buffer).unwrap();

  buffer
}

/// Reads the `library/build.gradle.kts` file and parses it as KTS
/// and returns the value of the `version` property as string.
pub fn get_current_version() -> String {
  let content = read_build_gradle_kts();

  // Find the `version = "x.y.z"` line.
  let version_line = content
    .lines()
    .find(|line| line.contains("version"))
    .expect("'build.gradle.kts' is missing 'version' property.");

  let version = version_line
    .split("\"")
    .nth(1)
    .expect("'version' should be wrapped in double quotes.");

  version.to_string()
}

fn bump_build_gradle_kts(old_version: &str, new_version: &str) {
  let content = read_build_gradle_kts();

  let from = format!("version = \"{}\"", old_version);
  let to = format!("version = \"{}\"", new_version);

  let content = content.replace(&from, &to);

  let mut file = File::create("library/build.gradle.kts").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

fn bump_readme(old_version: &str, new_version: &str) {
  let content = read_readme();
  let artifact_id = find_between(&content, "<artifactId>", "</artifactId>");

  // replace for maven section
  let from = format!("<version>{}</version>", old_version);
  let to = format!("<version>{}</version>", new_version);
  let content = content.replace(&from, &to);

  // replace for gradle (kotlin) section
  let from = format!(
    "implementation(\"ink.literate:{artifact_id}:{}\")",
    old_version
  );
  let to = format!(
    "implementation(\"ink.literate:{artifact_id}:{}\")",
    new_version
  );
  let content = content.replace(&from, &to);

  // replace for gradle section
  let from = format!(
    "implementation 'ink.literate:{artifact_id}:{}'",
    old_version
  );
  let to = format!(
    "implementation 'ink.literate:{artifact_id}:{}'",
    new_version
  );
  let content = content.replace(&from, &to);

  let mut file = File::create("README.md").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

pub fn bump_version(new_version: &str) {
  let old_version = get_current_version();

  bump_build_gradle_kts(&old_version, new_version);
  bump_readme(&old_version, new_version);
}

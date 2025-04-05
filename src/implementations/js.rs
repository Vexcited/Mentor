use std::{
  fs::File,
  io::{Read, Write},
  process::Command,
};

/// Concerning JS/TS implementations, we should only run `pnpm run checks` command.
/// It'll run `tsc`, `eslint` and sometimes some tests if they're available.
pub fn run_checks() {
  let output = Command::new("pnpm")
    .arg("run")
    .arg("checks")
    .output()
    .expect("failed to run pnpm command, make sure pnpm is installed globally on your machine");

  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stdout);
    panic!("failed to run `pnpm run checks` command, see the following stack trace:\n\n{error}");
  }
}

fn open_package_json() -> File {
  File::open("package.json").unwrap()
}

/// Reads the `package.json` file and parses it as JSON
/// and returns the value of the `version` property as string.
pub fn get_current_version() -> String {
  let file = open_package_json();

  let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");

  let version = json
    .get("version")
    .expect("'package.json' is missing 'version' property.");

  version.as_str().unwrap().to_string()
}

/// Edits the `package.json` file and updates the value of the `version` property.
/// We can't use serde for this as it'll mess up the formatting.
/// Instead, we manually replace the version in the content.
pub fn bump_version(version: &str) {
  let mut file = open_package_json();

  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  let from = format!("\"version\": \"{}\"", get_current_version());
  let to = format!("\"version\": \"{}\"", version);

  let content = content.replace(&from, &to);

  let mut file = File::create("package.json").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

use colored::Colorize;
use std::process::{Command, Output};

pub fn git(args: &[&str]) -> Output {
  let log = format!("+> git {}", args.join(" "));
  println!("{}", log.bright_black());

  Command::new("git")
    .args(args)
    .output()
    .expect("failed to run git command, make sure git is installed on your machine")
}

// get every commit message between the two versions
pub fn diff(old_version: &str, new_version: &str) -> String {
  let references = format!("{old_version}..{new_version}");

  let output = git(&["log", "--oneline", "--pretty=format:%s (%h)", &references]);
  let output = String::from_utf8_lossy(&output.stdout).to_string();

  let mut lines = output.lines().collect::<Vec<_>>();

  // reverse the lines to get the oldest commit first
  lines.reverse();

  // remove the last line because it's the version commit
  lines.pop();

  lines
    .iter()
    .map(|line| format!("* {}", line))
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn origin_url() -> String {
  let output = git(&["remote", "get-url", "origin"]);
  let url = String::from_utf8_lossy(&output.stdout);

  url.trim().to_string()
}

pub fn branch_name() -> String {
  let output = git(&["rev-parse", "--abbrev-ref", "HEAD"]);
  let branch_name = String::from_utf8_lossy(&output.stdout);
  branch_name.trim().to_string()
}

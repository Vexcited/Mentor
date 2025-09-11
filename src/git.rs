use anyhow::Result;
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
    .map(|line| format!("* {line}"))
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

pub fn is_repo_dirty() -> Result<bool> {
  let output = git(&["status", "--porcelain"]);

  if !output.status.success() {
    return Err(anyhow::anyhow!("failed to check repository status"));
  }

  // If there's any output, the repo is dirty
  Ok(!output.stdout.is_empty())
}

pub fn is_behind_upstream(branch_name: &str) -> Result<bool> {
  let fetch = git(&["fetch"]);
  if !fetch.status.success() {
    return Err(anyhow::anyhow!("failed to fetch from remote"));
  }

  let upstream = format!("origin/{branch_name}");
  let output = git(&["rev-list", "--count", &format!("HEAD..{upstream}")]);

  if !output.status.success() {
    return Err(anyhow::anyhow!(
      "failed to check if branch is behind remote"
    ));
  }

  let behind_count = String::from_utf8_lossy(&output.stdout)
    .trim()
    .parse::<u32>()
    .unwrap_or(0);

  Ok(behind_count > 0)
}

/// Get all tags of the repository, latest first to oldest tag.
pub fn tags() -> Vec<String> {
  let output = git(&["tag", "--sort=-v:refname"]);
  let output = String::from_utf8_lossy(&output.stdout).to_string();

  output.lines().map(|line| line.into()).collect::<Vec<_>>()
}

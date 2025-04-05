use crate::git;
use std::fmt;

pub enum Language {
  Rust,
  Kotlin,
  JsTs,
}

impl fmt::Display for Language {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Rust => "Rust",
        Self::Kotlin => "Kotlin",
        Self::JsTs => "JS/TS",
      }
    )
  }
}

impl Language {
  pub fn from_branch_name(name: &str) -> Self {
    match name {
      "js" => Language::JsTs,
      "rust" => Language::Rust,
      "kotlin" => Language::Kotlin,
      _ => panic!("unknown branch, make sure to checkout to a valid branch"),
    }
  }

  pub fn to_branch_name(&self) -> &str {
    match self {
      Language::JsTs => "js",
      Language::Rust => "rust",
      Language::Kotlin => "kotlin",
    }
  }
}

pub fn detect_language() -> Language {
  let output = git(&["rev-parse", "--abbrev-ref", "HEAD"]);

  let branch_name = String::from_utf8_lossy(&output.stdout);
  let branch_name = branch_name.trim();

  if branch_name == "main" || branch_name == "index" {
    panic!("you can't release from the primary branch, please checkout to an implementation branch, such as 'js' or 'kotlin'");
  }

  Language::from_branch_name(branch_name)
}

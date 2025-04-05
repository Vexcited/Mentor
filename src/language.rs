use crate::implementations::{js, kotlin, rust};
use anyhow::Result;
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

pub fn detect_language() -> Result<Language> {
  if let Ok(file) = js::open_package_json() {
    if file.metadata().is_ok() {
      return Ok(Language::JsTs);
    }
  }

  if let Ok(file) = kotlin::open_build_gradle_kts() {
    if file.metadata().is_ok() {
      return Ok(Language::Kotlin);
    }
  }

  if let Ok(file) = rust::open_cargo_toml() {
    if file.metadata().is_ok() {
      return Ok(Language::Rust);
    }
  }

  Err(anyhow::anyhow!(
    "Couldn't detect the language, make sure to checkout to a valid branch."
  ))
}

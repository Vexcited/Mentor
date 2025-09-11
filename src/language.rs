use crate::{
  implementations::{js, kotlin, rust, swift},
  utils::file_exists,
};
use anyhow::Result;
use std::fmt;

pub enum Language {
  Rust,
  Kotlin,
  JsTs,
  Swift,
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
        Self::Swift => "Swift",
      }
    )
  }
}

pub fn detect_language() -> Result<Language> {
  if file_exists(js::PACKAGE_JSON) {
    return Ok(Language::JsTs);
  }

  if file_exists(kotlin::BUILD_GRADLE_KTS) {
    return Ok(Language::Kotlin);
  }

  if file_exists(rust::CARGO_TOML) {
    return Ok(Language::Rust);
  }

  if file_exists(swift::PACKAGE_SWIFT) {
    return Ok(Language::Swift);
  }

  Err(anyhow::anyhow!(
    "couldn't detect the language, probably not supported"
  ))
}

use crate::{git, version::DEFAULT};

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

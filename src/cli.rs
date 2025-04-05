use crate::version::{bump_version, string_to_version, version_to_string};
use dialoguer::Select;

pub fn prompt_new_version(current_version: &str) -> String {
  let version = string_to_version(current_version);

  let selection = Select::new()
    .with_prompt(format!(
      "We're currently at {current_version}, next bump should be a"
    ))
    .items(&[
      format!("major ({})", version_to_string(bump_version(&version, 0))),
      format!("minor ({})", version_to_string(bump_version(&version, 1))),
      format!("patch ({})", version_to_string(bump_version(&version, 2))),
    ])
    .default(2) // default to patch.
    .interact()
    .unwrap();

  version_to_string(bump_version(&version, selection))
}

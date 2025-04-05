use colored::Colorize;
use spinners::{Spinner, Spinners};

mod utils;
mod version;

mod git;
use git::{diff, git};

mod github;
use github::open_create_release;

mod cli;
use cli::prompt_new_version;

mod language;
use language::{detect_language, Language};

mod implementations;
use implementations::js;
use implementations::kotlin;

const UNSUPPORTED_LANGUAGE: &str = "unsupported language, make sure to checkout to a valid branch";

fn main() {
  // Show a warning message, just in case.
  println!("{}\n", "Welcome, please note that this tool is only intended to be used within the LiterateInk organization, since it expects a specific repository structure and provides no way to configure any feature.".yellow());

  //
  // Detect the language of the current implementation.
  //

  let language = detect_language();
  println!("Automatically detected {language} implementation");

  {
    // Run checks depending on the language.
    let mut spinner = Spinner::new(
      Spinners::Dots,
      "Running checks for this specific implementation...".into(),
    );

    match language {
      Language::JsTs => js::run_checks(),
      Language::Kotlin => kotlin::run_checks(),
      _ => panic!("{UNSUPPORTED_LANGUAGE}"),
    };

    spinner.stop_with_message("Checks are passing, ready to release !".green().to_string());
  }

  //
  // Read the current version.
  //

  let old_version = match language {
    Language::JsTs => js::get_current_version(),
    Language::Kotlin => kotlin::get_current_version(),
    _ => panic!("{UNSUPPORTED_LANGUAGE}"),
  };

  //
  // Bump the version, by asking the user.
  //

  let new_version = prompt_new_version(&old_version);

  match language {
    Language::JsTs => js::bump_version(&new_version),
    Language::Kotlin => kotlin::bump_version(&new_version),
    _ => panic!("{UNSUPPORTED_LANGUAGE}"),
  }

  //
  // Commit, tag and push to origin.
  //

  let commit_message = format!("chore: release v{new_version}");
  let tag_message = format!("Release v{new_version}");
  let tag_name = format!("{}-v{new_version}", language.to_branch_name());

  let commands = vec![
    // NOTE: not very safe to add everything, might be great in the future to
    // have this as a separate function depending on the language.
    vec!["add", "."],
    vec!["commit", "-m", &commit_message],
    vec!["tag", "-a", &tag_name, "-m", &tag_message],
    vec!["push", "origin", language.to_branch_name(), "--tags"],
  ];

  for command in commands {
    let output = git(&command);

    if !output.status.success() {
      let error = String::from_utf8_lossy(&output.stdout);
      panic!("{error}");
    }
  }

  //
  // Make a release on GitHub.
  //

  let release_body = diff(&old_version, &new_version);
  let release_name = format!("{} v{new_version}", language.to_branch_name());
  open_create_release(release_body, tag_name, release_name);

  // Show an exit message, the CLI has finished its job.
  println!("{}", "Release is now being distributed !".green());
}

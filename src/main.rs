use easychangedirectory as ed;

use ed::error::PrintError;

fn main() {
  let cli = ed::cli();

  cli.match_options();

  let action = match ed::app() {
    Ok(action) => action,
    Err(e) => {
      e.eprintln();
      return;
    }
  };

  let action_path = match action {
    ed::Action::Change(cd_path) => {
      cli.match_temp_path(&cd_path);
      cd_path
    }
    ed::Action::Keep(current) => current,
    ed::Action::Print(print_path) => {
      println!("{}", print_path.display());
      print_path
    }
  };

  if let Ok(config) = ed::Config::new() {
    if config.is_pwd() {
      println!("Now: {}", action_path.display());
    }
    if config.is_log() {
      println!("Log output location: {}", ed::log::LogOutput::path().display());
    }
  }
}

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate mongodb;

extern crate juniper;

use clap::{App, Arg};
use std::sync::RwLock;
use std::ops::Deref;

mod logging;
mod settings;
use settings::Settings;

struct AppArgs {
  config: String,
  verbosity: u64
}

impl AppArgs {
  fn new() -> Self {
    let arguments = App::new("Auditer")
    .about("Chimney Rock Auditing Service")
    .version(crate_version!())
    .arg(
      Arg::with_name("config")
        .long("config")
        .short("c")
        .value_name("FILE")
        .default_value("./default-config.yaml")
        .help("Sets a custom config file")
        .takes_value(true)
    )
    .arg(
      Arg::with_name("verbose")
        .long("verbose")
        .short("v")
        .multiple(true)
        .help("Increases logging verbosity each use for up to 3 times")
    ).get_matches();

    AppArgs {
      config:    String::from(arguments.value_of("config").expect("invalid config value")),
      verbosity: arguments.occurrences_of("verbose")
    }
  }
}

lazy_static! {
  static ref APP_ARGS: AppArgs      = AppArgs::new();
  static ref APP_SETTINGS: Settings = Settings::new();
}

pub trait Join {
  /// # Example
  /// ```
  /// use array_tool::vec::Join;
  ///
  /// vec![1,2,3].join(",");
  /// ```
  ///
  /// # Output
  /// ```text
  /// "1,2,3"
  /// ```
  fn join(&self, joiner: &'static str) -> String;
}

impl<T: ToString> Join for Vec<T> {
  fn join(&self, joiner: &'static str) -> String {
    let mut out = String::from("");
    for x in 0..self.len() {
      out.push_str(&self[x].to_string());
      if x < self.len()-1 {
        out.push_str(&joiner)
      }
    }
    out
  }
}

fn main() -> std::io::Result<()> {
  // Ensure all statics are valid
  let (_, _) = (APP_ARGS.deref(), APP_SETTINGS.deref());

  logging::init().expect("Failed to initialize logging.");
  
  debug!("Loaded configuration => {:#?}", *APP_SETTINGS);

  Ok(())
}

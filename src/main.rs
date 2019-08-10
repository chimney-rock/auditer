#![feature(async_await)]
#![allow(dead_code)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

// #[macro_use]
extern crate mongodb;

use clap::{App, Arg};
use std::ops::Deref;

mod db;
mod grpc;
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

fn main() {
  if let Err(ref _err) = run() {
    std::process::exit(1);
  }
}

fn run() -> Result<(), failure::Error> {
  // Ensure all statics are valid
  let (_, _) = (APP_ARGS.deref(), APP_SETTINGS.deref());

  logging::init().expect("Failed to initialize logging.");
  
  grpc::start_server()?;
  Ok(())
}

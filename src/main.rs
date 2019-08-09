#![feature(async_await)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate mongodb;

extern crate juniper;

use clap::{App, Arg};
use std::ops::Deref;

mod db;
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

use mongodb::doc;
use wither::prelude::*;

fn run() -> Result<(), failure::Error> {
  // Ensure all statics are valid
  let (_, _) = (APP_ARGS.deref(), APP_SETTINGS.deref());

  logging::init().expect("Failed to initialize logging.");
  
  // debug!("Loaded configuration => {:#?}", *APP_SETTINGS);

  let database = db::Database::new()?;
  database.sync_all()?;

  let pool = database.pool.get()?;

  let ghetto_fabulous = doc! {
    "username": "nater540",
    "roles": ["admin", "unicorn"],
    "favorite_colors": ["yellow"]
  };

  let mut object_version = db::Version::new(None, ghetto_fabulous);
  object_version.save(pool.clone(), None)?;

  debug!("Work Complete!");
  Ok(())
}

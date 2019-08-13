use config::{Config, File, Environment};
use std::net::SocketAddr;
use serde::Deserialize;

use crate::APP_ARGS;

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub grpc_listener: Listener,
  pub amqp: Amqp,
  pub database: Database
}

#[derive(Debug, Deserialize)]
pub struct Listener {
  pub address: SocketAddr,
  pub backlog: Option<i16>,
  pub workers: Option<i16>,

  pub private_key: Option<String>,
  pub cert: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Amqp {
  pub address: SocketAddr,
  pub queue: String,
  pub private_key: Option<String>,
  pub cert: Option<String>
}

#[derive(Debug, Deserialize)]
pub enum DatabaseAdapter {
  MongoDB
}

#[derive(Debug, Deserialize)]
pub struct Database {
  pub adapter: DatabaseAdapter,
  pub name: String,
  pub host: String,
  pub port: Option<u16>,
  pub username: String,
  pub password: String,
  pub pool: Option<usize>
}

impl Settings {
  pub fn new() -> Self {
    let mut cfg = Config::new();

    cfg.merge(File::with_name(&APP_ARGS.config).required(false))
      .expect("Unable to load config file.");
    cfg.merge(Environment::with_prefix("auditer").separator("_"))
      .expect("Unable to use environment variables for configuration.");

    // Deserialize and freeze the entire configuration
    cfg.try_into().unwrap()
  }
}

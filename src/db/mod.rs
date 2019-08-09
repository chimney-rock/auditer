use r2d2::Pool;
use wither::prelude::*;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
// use mongodb::{db::{Database as MongoDatabase, ThreadedDatabase}, Error};

use crate::APP_SETTINGS;

mod version;
pub use version::Version;

#[derive(Clone)]
pub struct Database {
  pub pool: Pool<MongodbConnectionManager>
}

impl Database {
  pub fn new() -> Result<Self, failure::Error> {
    let manager = MongodbConnectionManager::new(
      ConnectionOptions::builder()
        .with_host(
          &APP_SETTINGS.database.host,
          APP_SETTINGS.database.port.unwrap_or(27017)
        )
        .with_db(&APP_SETTINGS.database.name)
        .with_auth(
          &APP_SETTINGS.database.username,
          &APP_SETTINGS.database.password
        )
        .build()
    );

    let pool = Pool::builder().build(manager)?;
    Ok(Database { pool })
  }

  pub fn sync_all(&self) -> Result<(), failure::Error> {
    let pool = self.pool.get()?;
    version::Version::sync(pool.clone())?;
    Ok(())
  }
}

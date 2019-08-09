use uuid::Uuid;
use wither_derive::Model;
use serde::{Serialize, Deserialize};
use mongodb::{coll::options::IndexModel, oid::ObjectId};

#[derive(Debug, Model, Serialize, Deserialize)]
pub struct Version {
  #[serde(rename="_id", skip_serializing_if="Option::is_none")]
  pub id: Option<ObjectId>,

  pub object_id: Uuid,
  pub object_data: bson::Document
}

impl Version {
  pub fn new(object_id: Option<Uuid>, object_data: bson::Document) -> Self {
    Version {
      id: None,
      object_id: object_id.unwrap_or(Uuid::new_v4()),
      object_data
    }
  }
}

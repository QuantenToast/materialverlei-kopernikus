use mongodb::bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Material {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic: Option<String>,
    pub num_available: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MaterialRes {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic: Option<String>,
    pub num_available: u32,
}

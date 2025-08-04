use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::clone::Clone;
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub tasks: Vec<ObjectId>
}
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::clone::Clone;
use bson::oid::ObjectId;

#[derive (Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<ObjectId>,
    pub taskname: String,
    pub user_id: ObjectId,
    pub status: String,   
}
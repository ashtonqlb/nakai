use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
        
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File<> {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        rename = "createdAt"
    )]
    pub created_at: DateTime<Utc>,
    pub uploader: String,
    pub filetype: String
}
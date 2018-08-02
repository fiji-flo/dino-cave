use serde_json::Value;

use std::collections::HashMap;
use std::sync::Mutex;

pub type ProfileStore = Mutex<HashMap<String, Value>>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserId {
    pub user_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateId {
    pub update_id: String,
}

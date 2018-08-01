use base64::decode;
use rocket_contrib::{Json, Value};
use std::str::from_utf8;

use data::*;

#[get("/<update_id>")]
pub fn cis_status(update_id: String) -> Json<Value> {
    let user_id = decode(&update_id).ok()
        .and_then(|bytes| from_utf8(&bytes).map(String::from).ok())
        .map(|user_id| UserId { user_id });
    match user_id {
        Some(user_id) => Json(json!(user_id)),
        None => Json(Value::Null)
    }
}
use base64::decode;
use rocket_contrib::Json;
use std::str::from_utf8;

use crate::data::*;

#[get("/<update_id>")]
pub fn cis_status(update_id: String) -> Option<Json> {
    let user_id = decode(&update_id)
        .ok()
        .and_then(|bytes| from_utf8(&bytes).map(String::from).ok())
        .map(|user_id| UserId { user_id });
    user_id.map(|user_id| Json(json!(user_id)))
}

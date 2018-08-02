use base64::encode;
use json_patch::merge;
use rocket::response::status::BadRequest;
use rocket::State;
use rocket_contrib::{Json, Value};

use crate::data::*;

#[post("/", format = "application/json", data = "<msg>")]
pub fn cis_update(msg: Json, store: State<(ProfileStore)>) -> Result<Json, BadRequest<Json>> {
    let profile_update = msg.into_inner();
    match update(&profile_update, store) {
        Some(update_id) => Ok(Json(json!(update_id))),
        None => Err(BadRequest(Some(Json(
            json!({ "error": "unable to process update" }),
        )))),
    }
}

fn update(profile_update: &Value, store: State<(ProfileStore)>) -> Option<UpdateId> {
    let user_id = profile_update
        .as_object()
        .and_then(|o| o.get("user_id"))
        .and_then(|id| id.as_str())
        .map(String::from)?;
    let update_id = encode(&user_id);
    store.lock().ok().map(|mut profiles| {
        profiles
            .get_mut(&user_id)
            .map(|profile| merge(profile, &profile_update))
    })?;
    Some(UpdateId { update_id })
}

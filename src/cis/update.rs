use base64::encode;
use json_patch::merge;
use rocket::State;
use rocket_contrib::{Json, Value};

use data::*;

#[post("/", format = "application/json", data = "<msg>")]
pub fn cis_update(msg: Json, ctx: State<(Ctx)>) -> Json<Value> {
    let profile_update = msg.into_inner();
    match update(&profile_update, ctx) {
        Some(update_id) => Json(json!(update_id)),
        None => Json(Value::Null)
    }
}

fn update(profile_update: &Value, ctx: State<(Ctx)>) -> Option<UpdateId> {
    let user_id = profile_update
        .as_object()
        .and_then(|o| o.get("user_id"))
        .and_then(|id| id.as_str())
        .map(String::from)?;
    let update_id = encode(&user_id);
    ctx.ctx.lock().ok().map(|mut profiles| {
        profiles
            .get_mut(&user_id)
            .map(|profile| merge(profile, &profile_update))
    })?;
    Some(UpdateId { update_id })
}
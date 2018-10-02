use crate::data::*;
use rocket::response::status::NotFound;
use rocket::State;
use rocket_contrib::Json;

#[get("/<user_id>")]
pub fn person_api(user_id: String, store: State<ProfileStore>) -> Result<Json, NotFound<Json>> {
    store
        .lock()
        .map_err(|e| format!("{}", e))
        .and_then(|profiles| {
            profiles
                .get(&user_id)
                .cloned()
                .ok_or_else(|| "profile not found".to_owned())
        })
        .map(Json)
        .map_err(|e| NotFound(Json(json!({ "error": e }))))
}

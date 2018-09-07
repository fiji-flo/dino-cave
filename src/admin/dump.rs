use crate::data::ProfileStore;
use rocket::response::status::BadRequest;
use rocket::State;
use rocket_contrib::Json;

#[get("/")]
pub fn dump(store: State<ProfileStore>) -> Result<Json, BadRequest<Json>> {
    store
        .lock()
        .map_err(|e| BadRequest(Some(Json(json!({ "error": format!("{}", e) })))))
        .map(|s| Json(json!(s.clone())))
}


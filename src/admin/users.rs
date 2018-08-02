use crate::data::ProfileStore;
use rocket::response::status::BadRequest;
use rocket::State;
use rocket_contrib::Json;

#[get("/")]
pub fn users(store: State<ProfileStore>) -> Result<Json, BadRequest<Json>> {
    let user_ids = store
        .lock()
        .map_err(|e| BadRequest(Some(Json(json!({ "error": format!("{}", e) })))))?
        .keys()
        .cloned()
        .collect::<Vec<String>>();
    Ok(Json(json!(user_ids)))
}

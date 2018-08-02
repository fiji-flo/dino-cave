use crate::cis::vault::write_profiles;
use crate::data::ProfileStore;
use rocket::response::status::BadRequest;
use rocket::State;
use rocket_contrib::{Json, Value};

#[get("/")]
pub fn persist(store: State<ProfileStore>) -> Result<Json, BadRequest<Json>> {
    let profiles = store
        .lock()
        .unwrap()
        .values()
        .cloned()
        .collect::<Vec<Value>>();
    let profiles_json = Value::from(profiles);
    match write_profiles(&profiles_json) {
        Ok(_) => Ok(Json(json!({ "success": true }))),
        Err(e) => Err(BadRequest(Some(Json(json!({ "error": e }))))),
    }
}

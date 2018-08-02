use crate::data::ProfileStore;
use rocket::State;
use rocket_contrib::Json;

#[get("/")]
pub fn users(store: State<ProfileStore>) -> Json {
    let user_ids = store
        .lock()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<String>>();
    rocket_contrib::Json(json!(user_ids))
}

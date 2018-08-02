use data::*;
use rocket::response::status::NotFound;
use rocket::State;
use rocket_contrib::{Json, Value};

#[get("/<user_id>")]
pub fn person_api(user_id: String, ctx: State<Ctx>) -> Result<Json, NotFound<Json>> {
    ctx.ctx
        .lock()
        .map_err(|e| format!("{}", e))
        .and_then(|profiles| {
            profiles
                .get(&user_id)
                .map(|p| p.clone())
                .ok_or_else(|| "profile not found".to_owned())
        })
        .map(Json)
        .map_err(|e| NotFound(Json(json!({ "error": e }))))
}

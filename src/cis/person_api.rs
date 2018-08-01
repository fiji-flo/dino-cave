use rocket_contrib::{Json, Value};
use rocket::State;
use data::*;

#[get("/<user_id>")]
pub fn person_api(user_id: String, ctx: State<Ctx>) -> Json<Value> {
    let profile = ctx.ctx.lock().unwrap().get(&user_id).unwrap().clone();
    Json(profile)
}


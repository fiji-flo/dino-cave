use crate::data::ProfileStore;
use maud::{html, Markup};
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

#[get("/")]
pub fn users_html(store: State<ProfileStore>) -> Markup {
    match store.lock() {
        Ok(profiles) => htmlify(&profiles.keys().cloned().collect::<Vec<String>>()),
        _ => html! { h1 { "Error" } },
    }
}

fn htmlify(users: &[String]) -> Markup {
    html! {
    ul {
        @for user in users {
            li {
                a href={"/profile/" (user)} { (user) }
            }
        }
    }
    }
}

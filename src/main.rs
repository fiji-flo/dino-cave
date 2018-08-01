#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate json_patch;

mod cis;
mod data;

use rocket::config::{Config, Environment};
use rocket::State;

use std::path::PathBuf;
use std::sync::Mutex;

use cis::vault::*;
use data::*;

#[get("/")]
fn users(ctx: State<Ctx>) -> rocket_contrib::Json<rocket_contrib::Value> {
    let user_ids = ctx
        .ctx
        .lock()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<String>>();
    rocket_contrib::Json(json!(user_ids))
}

fn main() {
    let profile_json = read_profiles(&PathBuf::from("/tmp/profiles.json")).unwrap();
    let profiles = index_profiles_by_user_id(&profile_json).unwrap();
    let ctx = Ctx {
        ctx: Mutex::new(profiles),
    };

    let config = Config::build(Environment::Staging)
        .port(8888)
        .finalize().unwrap();
    rocket::custom(config, true)
        .mount("/cisUpdate", routes![cis::update::cis_update])
        .mount("/cisStatus/", routes![cis::status::cis_status])
        .mount("/personApi/", routes![cis::person_api::person_api])
        .mount("/users/", routes![users])
        .manage(ctx)
        .launch();
}

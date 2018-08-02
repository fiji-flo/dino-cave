#![feature(rust_2018_preview)]
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

mod admin;
mod cis;
mod data;

use rocket::config::{Config, Environment};

use std::sync::Mutex;

use crate::cis::vault::*;

fn main() {
    let profile_json = read_profiles().unwrap();
    let profiles = index_profiles_by_user_id(&profile_json).unwrap();
    let store = Mutex::new(profiles);

    let config = Config::build(Environment::Staging)
        .port(8888)
        .finalize()
        .unwrap();
    rocket::custom(config, true)
        .mount("/cisUpdate", routes![cis::update::cis_update])
        .mount("/cisStatus/", routes![cis::status::cis_status])
        .mount("/personApi/", routes![cis::person_api::person_api])
        .mount("/admin/users", routes![admin::users::users])
        .mount("/admin/persist", routes![admin::persist::persist])
        .manage(store)
        .launch();
}

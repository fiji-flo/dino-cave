#![feature(rust_2018_preview)]
#![feature(plugin)]
#![feature(proc_macro_non_items)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate json_patch;
#[macro_use]
extern crate lazy_static;

mod admin;
mod cis;
mod data;

use rocket::config::{Config, Environment};

use std::sync::Mutex;

use crate::cis::vault::*;

fn main() -> Result<(), String> {
    let profile_json = read_profiles()?;
    let profiles = index_profiles_by_user_id(&profile_json)?;
    let store = Mutex::new(profiles);

    let config = Config::build(Environment::Staging)
        .port(8888)
        .finalize()
        .map_err(|e| format!("{}", e))?;
    rocket::custom(config, true)
        .mount("/cisUpdate", routes![cis::update::cis_update])
        .mount("/cisStatus/", routes![cis::status::cis_status])
        .mount("/personApi/", routes![cis::person_api::person_api])
        .mount("/admin/dump", routes![admin::dump::dump])
        .mount("/admin/users", routes![admin::users::users])
        .mount("/admin/persist", routes![admin::persist::persist])
        .manage(store)
        .launch();
    Ok(())
}

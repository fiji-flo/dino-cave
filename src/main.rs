#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate base64;
extern crate serde_json;
extern crate chashmap;

use base64::{decode, encode};

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str;
use std::sync::Arc;

use actix_web::http::Method;
use actix_web::{dev::Handler, error, server, App, HttpMessage, HttpRequest, HttpResponse, Json};
use chashmap::CHashMap;
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserId {
    user_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateId {
    update_id: String,
}

fn echo_post(data: Json<Value>) -> actix_web::Result<Json<Value>> {
    println!("Received: {}", data);
    Ok(data)
}

fn query_update(data: Json<Value>) -> actix_web::Result<Json<Value>> {
    println!("{:?}", data);
    let user_id = data.as_object()
        .and_then(|o| o.get("user_id"))
        .and_then(|id| id.as_str())
        .map(|s| s.to_owned());
    println!("Received update for {:?}", user_id);
    Ok(user_id
        .map(|id| encode(&id))
        .map(|id| UpdateId { update_id: id })
        .and_then(|id| serde_json::to_value(id).ok())
        .map(Json)
        .unwrap_or_else(|| Json(Value::Null)))
}

fn status(req: HttpRequest) -> actix_web::Result<Json<Value>> {
    let user_id = req.match_info().get("id");
    println!("Sending status for for {:?}", user_id);
    Ok(user_id
        .and_then(|id| decode(&id).ok())
        .and_then(|bytes| str::from_utf8(&bytes).map(String::from).ok())
        .map(|id| UserId { user_id: id })
        .and_then(|id| serde_json::to_value(id).ok())
        .map(Json)
        .unwrap_or_else(|| Json(Value::Null)))
}

struct UsersHandler {
    users: Arc<Vec<Value>>,
}

impl<S> Handler<S> for UsersHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&mut self, _: HttpRequest<S>) -> Self::Result {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(Value::Array(self.users.to_vec()))
    }
}

struct PersonApiHandler {
    profiles: Arc<CHashMap<String, Value>>,
}

impl<S> Handler<S> for PersonApiHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&mut self, req: HttpRequest<S>) -> Self::Result {
        match *req.method() {
            Method::GET => {
        if let Some(id) = req.match_info().get("id") {
            if let Some(profile) = self.profiles.get(&String::from(id)) {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(profile.clone());
            }
        }
        HttpResponse::Ok().body("")

            },
            Method::POST => {
    let user_id = req.payload()
            .and_then(|o| o.get("user_id"))
        .and_then(|id| id.as_str())
        .map(|s| s.to_owned());
    println!("Received update for {:?}", user_id);
        HttpResponse::Ok().body("")
            }

        }
    }
}

fn main() -> Result<(), String> {
    let profile_json = read_profiles(&PathBuf::from("/tmp/profiles.json"))?;
    let profiles = Arc::new(index_profiles_by_user_id(&profile_json)?);
    let users = Arc::new(get_all_user_ids(&profile_json)?);
    server::new(move || {
        let profiles = Arc::clone(&profiles);
        let users = Arc::clone(&users);
        App::new()
            .resource("/echo", |r| r.method(Method::POST).with(echo_post))
            .resource("/cisUpdate", |r| r.method(Method::POST).with(query_update))
            .resource(r"/cisStatus/{id}", |r| r.method(Method::GET).f(status))
            .resource("/personApi/{id}", move |r| {
                r.h(PersonApiHandler { profiles })
            })
            .resource("/users", move |r| r.h(UsersHandler { users }))
    }).bind("127.0.0.1:8888")
        .expect("Can not bind to port 8888")
        .run();
    Ok(())
}

fn read_profiles(path: &Path) -> Result<Value, String> {
    let mut f = File::open(path).map_err(|e| format!("file not found: {}", e))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| format!("unable to read file: {}", e))?;
    serde_json::from_str(&contents).map_err(|e| format!("unable to load json: {}", e))
}

fn get_all_user_ids(profile_json: &Value) -> Result<Vec<Value>, String> {
    if let Some(profiles) = profile_json.as_array() {
        return Ok(profiles
            .into_iter()
            .filter_map(|p| {
                p.as_object()
                    .and_then(|o| o.get("user_id"))
                    .and_then(|id| id.as_str().map(String::from))
                    .map(Value::String)
            })
            .collect());
    }

    Err(String::from("nope"))
}
fn index_profiles_by_user_id(profile_json: &Value) -> Result<CHashMap<String, Value>, String> {
    if let Some(profiles) = profile_json.as_array() {
        return Ok(profiles
            .into_iter()
            .filter_map(|p| {
                p.as_object()
                    .and_then(|o| o.get("user_id"))
                    .and_then(|id| id.as_str().map(str::to_owned))
                    .map(|s| (s, p.clone()))
            })
            .collect::<CHashMap<String, Value>>());
    }
    Err(String::from("nope"))
}

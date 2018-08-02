use serde_json::{from_str, to_writer_pretty, Value};

use std::collections::HashMap;
use std::env::var;
use std::fs::File;
use std::io::prelude::*;

lazy_static! {
    static ref PROFILE_STORE: String =
        var("DC_PROFILE_STORE").unwrap_or_else(|_| String::from("/tmp/profiles.json")
    );
}

pub fn read_profiles() -> Result<Value, String> {
    let mut f = File::open(PROFILE_STORE.as_str()).map_err(|e| format!("file not found: {}", e))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| format!("unable to read file: {}", e))?;
    from_str(&contents).map_err(|e| format!("unable to load json: {}", e))
}

pub fn write_profiles(profiles: &Value) -> Result<(), String> {
    let f = File::create(PROFILE_STORE.as_str()).map_err(|e| format!("file not found: {}", e))?;
    to_writer_pretty(f, &profiles).map_err(|e| format!("unable to write to file: {}", e))
}

pub fn index_profiles_by_user_id(profile_json: &Value) -> Result<HashMap<String, Value>, String> {
    if let Some(profiles) = profile_json.as_array() {
        return Ok(profiles
            .into_iter()
            .filter_map(|p| {
                p.as_object()
                    .and_then(|o| o.get("user_id"))
                    .and_then(|id| id.as_str().map(str::to_owned))
                    .map(|s| (s, p.clone()))
            })
            .collect::<HashMap<String, Value>>());
    }
    Err(String::from("nope"))
}

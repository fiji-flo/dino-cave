use serde_json::{from_str, Value};

use std::collections::HashMap;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

pub fn read_profiles(path: &Path) -> Result<Value, String> {
    let mut f = File::open(path).map_err(|e| format!("file not found: {}", e))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| format!("unable to read file: {}", e))?;
    from_str(&contents).map_err(|e| format!("unable to load json: {}", e))
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

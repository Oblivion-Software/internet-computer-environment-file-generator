use serde_json::{from_reader, Value};
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};
use str_macro::*;

pub struct CanisterIds {
    pub foo_canister_id: String,
    pub bar_canister_id: String,
}

impl CanisterIds {
    pub fn to_content(&self) -> String {
        let environment = format!("pub const ENVIRONMENT: &str = \"{}\";\r", get_env());

        let foo_canister_id = format!(
            "pub const FOO_CANISTER_ID: &str = \"{}\";\r",
            self.foo_canister_id
        );

        let bar_canister_id = format!(
            "pub const BAR_CANISTER_ID: &str = \"{}\";\r",
            self.bar_canister_id
        );

        format!("{}{}{}", environment, foo_canister_id, bar_canister_id)
    }
}

pub fn get_env() -> String {
    let env = env::var("ENV");
    match env {
        Ok(_env) => _env,
        Err(_) => str!("local"),
    }
}

pub fn generate_canister_ids() {
    let mut env_path = str!(".dfx/local/canister_ids.json");
    if get_env() != str!("local") || get_env() != "" {
        env_path = str!("canister_ids.json");
    }
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let file_path = dir.parent().unwrap().parent().unwrap().join(env_path);
    let file = File::open(file_path).expect("file should open read only");
    let json: Value = from_reader(file).expect("file should be proper JSON");

    let canister_ids = CanisterIds {
        foo_canister_id: get_json_value(&json, str!("foo_canister")),
        bar_canister_id: get_json_value(&json, str!("bar_canister")),
    };

    let _ = fs::write("environment_settings.rs", canister_ids.to_content());
}

fn get_json_value(json: &Value, name: String) -> String {
    let mut return_value: String = String::from("");

    let json_value = json.get(name);

    match json_value {
        Some(value) => match value.get(get_env()) {
            Some(v) => match v {
                Value::String(_v) => return_value = _v.clone(),
                Value::Null => {}
                Value::Bool(_) => {}
                Value::Number(_) => {}
                Value::Array(_) => {}
                Value::Object(_) => {}
            },
            None => return_value = str!(""),
        },
        None => return_value = str!(""),
    }

    return_value
}

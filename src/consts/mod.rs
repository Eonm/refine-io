use std::env;
use serde_json::Value;
use serde_json;
use reqwest;
use std::error::Error;

lazy_static! {
    pub static ref REFINE_BASE_URL: String = env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".to_string());
    pub static ref RECORD_PATH: String = env::var("RECORD_PATH").unwrap_or("[\"_\"]".to_string());
    pub static ref REFINE_PROJECTS_INFO : Result<Value, reqwest::Error> = {
        let refine_base_url = env::var("OPEN_REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
        let command_url = format!("{}/command/core/get-all-project-metadata", refine_base_url);

        let response = reqwest::get(&command_url)?.text()?;
        let v: Value = serde_json::from_str(&response).expect("Failed to decode JSON");
        Ok(v)
    };
}

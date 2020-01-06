use std::env;

lazy_static! {
    pub static ref REFINE_BASE_URL: String = env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".to_string());
    pub static ref RECORD_PATH: String = env::var("RECORD_PATH").unwrap_or("[\"_\"]".to_string());
}

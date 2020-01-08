use std::env;

lazy_static! {
    pub static ref REFINE_BASE_URL: String = env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".to_string());
    pub static ref RECORD_PATH: String = env::var("RECORD_PATH").unwrap_or("[\"_\"]".to_string());
    pub static ref CHECK_ASYNC_INTERVAL: i32 = env::var("CHECK_ASYNC_INTERVAL").unwrap_or("100".to_string()).parse().expect("Failed to parse CHECK_ASYNC_INTERVAL. CHECK_ASYNC_INTERVAL must be an integer");
}

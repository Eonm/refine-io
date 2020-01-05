#[macro_use]
extern crate log;
extern crate clap;
extern crate open;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use env_logger::Env;
use std::error::Error;

mod consts;
mod utils;
mod cli;
use cli::cli;

mod refine;
pub use refine::*;

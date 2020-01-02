#[macro_use] 
extern crate log;
extern crate open;
extern crate clap;

use std::env;
use std::error::Error;
use url::{Url};
use env_logger::Env;

mod utils;

mod cli;
use cli::cli;

mod refine;
use refine::Refine;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli();

    if !matches.is_present("silent") {
        env_logger::from_env(Env::default().default_filter_or("info")).init();
    };

    let format = match matches.value_of("format").expect("format not set") {
        "csv" => "text/line-based/csv",
        "tsv" => "text/line-based/tsv",
        "xls" => "binary/text/xml/xlsx",
        "xlsx" => "binary/text/xml/xls",
        "json" => "text/json",
        "xml" => "text/xml",
        _ => ""
    };

    let input = matches.value_of("input").expect("input not set");
    let script = matches.value_of("script");
    let record_path = matches.value_of("record_path").expect("record-path not set");
    let project_name = matches.value_of("project_name").unwrap_or("");
   
    let export_format = match matches.value_of("export_format") {
        Some(f) => Some(f.to_string()),
        None => None
    };

    let data = match Url::parse(input) {
        Ok(_) => utils::download(input)?,
        Err(_) => utils::load(input)?
    };
    
    let mut r = Refine::new(format, record_path, script);
    
    let refine_result = r.refine(data, project_name, export_format)?;

    if matches.is_present("open") {
        info!("opening OpenRefine project");
        
        if open::that(format!("{}/project?project={}", env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".into()), refine_result.project_id.clone().expect("Expect project id"))).is_ok() {
            info!("OpenRefine opened");
        } else {
            warn!("Failed to OpenRefine project")
        }
    }
    Ok(())
}
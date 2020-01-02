#[macro_use]
extern crate log;
extern crate clap;
extern crate open;

use env_logger::Env;
use std::env;
use std::error::Error;
use url::Url;

mod utils;

mod cli;
use cli::cli;

mod refine;
use refine::{RefineInit, RefineProject};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli();

    if !matches.is_present("silent") {
        env_logger::from_env(Env::default().default_filter_or("info")).init();
    };

    let format = match matches.value_of("format").expect("format not set") {
        "csv" => "text/line-based/csv",
        "tsv" => "text/line-based/tsv",
        "json" => "text/json",
        "xml" => "text/xml",
        _ => "",
    };

    let data = match matches.value_of("input") {
        Some(input) => match Url::parse(input) {
            Ok(_) => utils::download(input)?,
            Err(_) => utils::load(input)?,
        },
        None => {
            use std::io::{self, Read};
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };
    let script = matches.value_of("script");

    let record_path = matches
        .value_of("record_path")
        .expect("record-path not set");

    let project_name = matches.value_of("project_name").unwrap_or("");

    let export_format = match matches.value_of("export_format") {
        Some(f) => Some(f.to_string()),
        None => None,
    };

    let mut refine = RefineInit::new(format, record_path, script);
    let project: RefineProject = refine.create_project(data, project_name)?;

    if matches.is_present("open_project") {
        info!("opening OpenRefine project");

        if open::that(format!(
            "{}/project?project={}",
            env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".into()),
            project.project_id.clone()
        ))
        .is_ok()
        {
            info!("OpenRefine opened");
        } else {
            warn!("Failed to OpenRefine project")
        }
    }

    if matches.is_present("script") {
        project.clone().apply_operations()?;
    }

    if let (Some(export_format), Some(output)) = (
        matches.value_of("export_format"),
        matches.value_of("output"),
    ) {
        project.clone().export(Some(export_format.into()))?;
    } else if let (Some(export_format), None) = (
        matches.value_of("export_format"),
        matches.value_of("output"),
    ) {
        project.clone().print(Some(export_format.into()))?;
    }

    Ok(())
}

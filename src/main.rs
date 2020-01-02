use chrono::prelude::*;
use std::error::Error;
use std::env;
use dotenv::dotenv;
use std::fs;

use std::io::copy;
use std::fs::File;

// #[macro_use] 
// extern crate serde;

use url::{Url, ParseError};

extern crate open;

use serde::Deserialize;

#[macro_use] 
extern crate log;

use log::{error};

use env_logger::Env;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Box<dyn Error>> {
       let matches = App::new("refine-io")
            .version("1.0")
            .author("eonm. <eon.mathis@gmail.com>")
            .about("Automatise la creation et l'export de projet OpenRefine")
            .arg(Arg::with_name("script")
                .short("s")
                .long("script")
                .value_name("FILE")
                .help("Script de transformation OpenRefine")
                .takes_value(true))
            .arg(Arg::with_name("record_path")
                .short("r")
                .long("record-path")
                .value_name("FILE")
                .help("")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("project_name")
                .short("n")
                .long("project-name")
                .value_name("NAME")
                .help("Nom du projet OpenRefine")
                .takes_value(true))
            .arg(Arg::with_name("fichier-sortie")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Fichier de sortie")
                .takes_value(true))
            .arg(Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Format de fichier")
                .possible_values(&["json", "xml", "csv", "tsv", "xls", "xlsx"])
                .required(true)
                .takes_value(true))
             .arg(Arg::with_name("export_format")
                .short("e")
                .long("export-format")
                .value_name("FORMAT")
                .help("Format d'export")
                .possible_values(&["csv", "tsv", "xls", "xlsx", "ods", "html"])
                .takes_value(true))
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FICHIER|URL")
                .help("Source")
                .takes_value(true))
            .arg(Arg::with_name("open")
                .long("open")
                .help("Ouvre le projet refine après sa création"))
            .arg(Arg::with_name("silent")
                .long("silent")
                .help("N'affiche pas les logs"))
            .get_matches();

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
        Ok(_) => download(input)?,
        Err(_) => load(input)?
    };
    
    // r#"["_","response","docs","_"]"#
    // '["_","response","docs","_"]'
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

#[derive(Debug, Deserialize, Clone)]
pub struct Refine<'a> {
    refine_script: Option<&'a str>,
    project_id: Option<String>,
    project_name: Option<String>,
    data_format: &'a str,
    record_path: &'a str
}

impl <'a>Refine<'a> {
    pub fn new(data_format: &'a str, record_path: &'a str, refine_script: Option<&'a str>) -> Refine<'a> {
        let refine = Refine {
            refine_script: refine_script,
            record_path: record_path,
            data_format: data_format,
            project_id: None,
            project_name: None,
        };

        refine
    }

    pub fn refine(&'a mut self, data: String, name: &'a str, format: Option<String>) -> Result<&'a mut Refine, Box<dyn Error>> {
        self.create_project(data, name)?
            .apply_operations()?
            .export(format)
    }

    fn create_project(&'a mut self, data: String, project_name: &'a str) -> Result<&mut Refine, Box<dyn Error>>{
        self.project_name = Some(format!("{} [{}]", Utc::now(), project_name));

        info!("Creating an OpenRefin project");
        let mut form = reqwest::multipart::Form::new()
            .text("project-name", self.project_name.clone().unwrap())
            .text("format", self.data_format.to_string())
            .text("options", format!("{}\"recordPath\":{} {}", "{", self.record_path.clone(), "}"));

        let project_file = reqwest::multipart::Part::text(data)
            .file_name("data.json")
            .mime_str("text/json")?;

        form = form.part("project-file", project_file);

        dotenv().ok();
        let refine_base_url = env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
        let create_project_url = format!("{}{}", refine_base_url, "/command/core/create-project-from-upload");

        let client = reqwest::Client::new();
        let response = client
            .post(&create_project_url)
            .multipart(form)
            .send()?;


        if let Some(project_id) = response.url().query_pairs().filter(|(param, _value)| param == "project").next() {
            info!("OpenRefine project created {}", project_id.1);
            self.project_id =  Some(project_id.1.into());
        }

        Ok(self)
    }

    pub fn apply_operations(&'a mut self) -> Result<&mut Refine, Box<dyn Error>> {
        dotenv().ok();
        if let (Some(project_id), Some(script)) = (self.project_id.clone(), self.refine_script.clone()) {

            info!("Applying script {} to OpenRefine project {}", script, project_id);
        
            let script_contents = fs::read_to_string(script)?;
            let params = [("project", &project_id), ("operations", &script_contents)];
            let refine_base_url = env::var("OPEN_REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
            let apply_operations_url = format!("{}{}", refine_base_url, "/command/core/apply-operations");

            let client = reqwest::Client::new();
            client
                .post(&apply_operations_url)
                .form(&params)
                .send()?;

            info!("Script applied");
        }

        Ok(self)
    }

    fn export(&'a mut self, format: Option<String>) -> Result<&mut Refine, Box<dyn Error>> {
        match format {
            Some(format) => {
                info!("exporting data");
                let refine_base_url = env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
                let create_project_url = format!("{}{}", refine_base_url, "/command/core/export-rows");
                
                use std::collections::HashMap;

                let mut params = HashMap::new();
                params.insert("project", self.project_id.clone().expect("Should have a project IDs"));
                params.insert("engine", r#"'{"facets": [], "mode": "row-based"}'"#.into());
                params.insert("format", format.clone());

                let client = reqwest::Client::new();
                let mut response = client
                    .post(&create_project_url)
                    .form(&params)                   
                    .send()?;

                let mut dest = {
                    let fname = format!("{}.{}", self.project_name.clone().expect("Should have a project ID"), format);
                    
                    info!("data will be located under: {}", fname);
                    File::create(fname)?
                };

                copy(&mut response, &mut dest)?;
                info!("data exported");
            },
            None => ()
        };

        Ok(self)
    }
}

fn download(url: &str) -> Result<String, Box<dyn Error>> {
    info!("downloading data");
    let data = reqwest::get(url)?
        .text()?;
    
    info!("data downloaded :\n\t{}", data);
    Ok(data)
}

fn load(path: &str) -> Result<String, Box<dyn Error>> {
    info!("loading data from {}", path);
    let data = fs::read_to_string(path)?;
    info!("data loaded :\n\t{}", data);
    Ok(data)
}
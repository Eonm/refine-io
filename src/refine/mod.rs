use url::Url;
use serde_json::Value;

use std::env;
use std::error::Error;
use std::io::{self, Read};

pub mod import;
pub mod process;
pub mod export;
pub mod delete;
use crate::refine::import::Import;

/// A struct for initialize a OpenRefine project
#[derive(Debug, Clone)]
pub struct RefineInit<'a> {
    pub refine_script: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub data_format: &'a str,
    pub record_path: &'a str,
}

impl<'a> RefineInit<'a> {
    pub fn create_project(data_source: Option<&'a str>, data_format: Option<&'a str>, project_name: Option<&'a str>, record_path: Option<&'a str>) -> Result<RefineProject, Box<dyn Error>> {
        match data_source {
            Some(source) => {
                info!("creating OpenRefine project");
                match Url::parse(source) {
                    Ok(_) => RefineInit::from_url(source, data_format.expect("data format is expected for URL import"), project_name, record_path),
                    Err(_) => {
                        RefineInit::from_file(source, project_name, record_path)
                    },
                }
            },
            None => {
                let data_format = match data_format {
                    Some(format) => {
                        if format == "json" || format == "xml" {
                           if let None = record_path {
                                error!("record path (--record-path) must be specified when data are JSON or XML format");
                                panic!("record path (--record-path) must be specified when data are JSON or XML format")
                            }
                        }
                        format
                    },
                    None => {
                        error!("data format (--format) must be specified when data are imported from STDIN");
                        panic!("data format (--format) must be specified when data are imported from STDIN")
                    },
                };
                
                println!("Waiting for user input. Press C^D to submit data");
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;

                info!("creating OpenRefine project");
                RefineInit::from_string(&buffer, data_format, project_name, record_path)
            }
        }
    }
}

/// A struct representing an OpenRefine project
#[derive(Debug, Clone)]
pub struct RefineProject {
    pub refine_script: Option<String>,
    pub project_id: String,
    pub project_name: String,
}

impl RefineProject {
    /// Load an OpenRefine project
    pub fn load(id: &str) -> Result<RefineProject, Box<dyn Error>> {
        info!("loading OpenRefine project {}", id);
        let refine_base_url = env::var("OPEN_REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
        let command_url = format!("{}/command/core/get-all-project-metadata", refine_base_url);

        let response = reqwest::get(&command_url)?.text()?;
        let v: Value = serde_json::from_str(&response)?;

        let project_name =  match v["projects"][id]["name"].as_str() {
            Some(value) => value,
            None => {
                error!("Failed to load project {}", id);
                panic!("Failed to load project")
            },
        };

        info!("OpenRefine project loaded");

        Ok(RefineProject {
            refine_script: None,
            project_id: id.into(),
            project_name: project_name.into(),
        })
    }
}

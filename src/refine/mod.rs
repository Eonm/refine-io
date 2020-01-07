use url::Url;

use std::error::Error;
use std::io::{self, Read};

pub mod import;
pub mod process;
pub mod export;
pub mod delete;
pub mod info;
use crate::refine::import::Import;
use crate::consts;

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
                    Ok(_) => {
                        match data_format {
                            Some(format) => {
                                RefineInit::from_url(source, format, project_name, record_path)
                            },
                            None => {
                                error!("data format (--format) is expected for URL import");
                                panic!("data format (--format) is expected for URL import")
                            },
                        }
                    },
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
                
                #[cfg(not(test))]
                println!("Waiting for user input. Press C^D to submit data");
                #[cfg(not(test))]
                let mut buffer = String::new();
                #[cfg(not(test))]
                io::stdin().read_to_string(&mut buffer)?;

                #[cfg(test)]
                let buffer = "test input\r\n".to_string();

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

        let projects_info = consts::REFINE_PROJECTS_INFO.as_ref().expect("Failed to get OpenRefine projects data");

        let project_name =  match projects_info["projects"][id]["name"].as_str() {
            Some(value) => value,
            None => {
                error!("Failed to load project {}", id);
                panic!("Failed to load project {}", id)
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::refine::delete::Delete;


   #[test]
   
    fn test_load() {
        let project = RefineInit::create_project(Some("playground/input.json"), Some("json"), None, Some(r#"["_"]"#)).expect("Failed to create project");
        RefineProject::load(&project.project_id).expect("Failed to load project");
        project.delete().expect("Failed to delete project");
    }

    #[test]
    fn create_project_from_stdin() {
        let project = RefineInit::create_project(None, Some("json"), None, Some(r#"["_"]"#)).expect("Failed to create project");
        project.delete().expect("Failed to delete project");
    }

    #[test]
    #[should_panic]
    fn test_url_import_requires_format() {
        let project = RefineInit::create_project(Some("http://127.0.0.1/playground/input.json"), None, None, Some(r#"["_"]"#)).expect("Failed to create project");
        project.delete().expect("Failed to delete project");
    }

    #[test]
    #[should_panic]
    fn test_stdin_requires_format() {
        let project = RefineInit::create_project(None, None, None, Some(r#"["_"]"#)).expect("Failed to create project");
        project.delete().expect("Failed to delete project");
    }

    #[test]
    #[should_panic]
    fn test_stdin_json_requires_reccord_path() {
        let project = RefineInit::create_project(None, Some("json"), None, None).expect("Failed to create project");
        project.delete().expect("Failed to delete project");
    }

    #[test]
    #[should_panic]
    fn test_stdin_xml_requires_reccord_path() {
        let project = RefineInit::create_project(None, Some("xml"), None, None).expect("Failed to create project");
        project.delete().expect("Failed to delete project");
    }
}

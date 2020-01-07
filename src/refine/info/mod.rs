use reqwest;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use crate::refine::{RefineInit, RefineProject};
use crate::consts;
use crate::utils;


use std::error::Error;

/// A struct for Deserialize an OpenRefine project info
#[derive(Debug, Deserialize)]
struct ProjectInfo {
    created: Option<String>,
    modified: Option<String>,
    name: Option<String>,
    tags: Option<Vec<String>>,
    creator: Option<String>,
    contributors: Option<String>,
    subject: Option<String>,
    description: Option<String>,
    rowCount: Option<usize>,
    title: Option<String>,
    version: Option<String>,
    license: Option<String>,
    homepage: Option<String>,
    image: Option<String>,
}

/// This trait allows you to get and print Info about an OpenRefine project
pub trait Info<'a> {
    /// Retrun info as struct
    fn get(self) -> Result<ProjectInfo, Box<dyn Error>>;
    /// Return info as String
    fn print_raw(self) -> Result<String, Box<dyn Error>>;
}

impl <'a>Info<'a> for RefineProject {
    fn get(self) -> Result<ProjectInfo, Box<dyn Error>> {
        let projects_info = consts::REFINE_PROJECTS_INFO.as_ref().expect("Failed to get projects info");
        let current_project_info : ProjectInfo = serde_json::from_value(projects_info["projects"][self.project_id].clone())?;

        Ok(current_project_info)
    }

    fn print_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(format!("{:#?}", self.get()?))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::refine::delete::Delete;
    use crate::refine::import::Import;

    #[test]
    fn test_get_info() {
        let project = RefineInit::create_project(Some("playground/input.json"), Some("json"), None, Some(r#"["_"]"#)).expect("Failed to create project");
        let info = project.clone().get();
        eprintln!("{:?}", info);

        project.delete().expect("Failed to delete project");
    }
}
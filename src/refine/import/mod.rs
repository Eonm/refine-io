use reqwest;
use chrono::prelude::*;

use crate::refine::{RefineInit, RefineProject};
use crate::consts;
use crate::utils;

use std::convert::AsRef;
use std::error::Error;

/// This trait allows you to create an OpenRefine project from a local file or an url
pub trait Import<'a> {
    /// Create an OpenRefine project based on local file
    fn from_file(path: &'a str, project_name: Option<&'a str>, record_path: Option<&'a str>) -> Result<RefineProject, Box<dyn Error>>;
    /// Create an OpenRefine project based on remote ressources
    fn from_url(url: &'a str, data_format: &'a str, project_name: Option<&'a str>, record_path: Option<&'a str>) -> Result<RefineProject, Box<dyn Error>>;
    /// Create an OpenRefine project from string
    fn from_string(data: &'a str, data_format: &'a str, project_name: Option<&'a str>, record_path: Option<&'a str>) -> Result<RefineProject, Box<dyn Error>>;
}

impl <'a>Import<'a> for RefineInit<'a> {
    fn from_file(path: &'a str, project_name: Option<&'a str>, record_path: Option<&'a str>) -> Result<RefineProject, Box<dyn Error>> {
        let project_name = project_name.unwrap_or(&format!("{}", Utc::now())).to_string();
        let record_path = record_path.unwrap_or(&consts::RECORD_PATH.to_string()).to_string();

        let submit_form = reqwest::multipart::Form::new()
            .text("project-name", project_name)
            .text(
                "options",
                format!("{}\"recordPath\":{} {}", "{", record_path.clone(), "}"),
            )
            .file("project-file", path)?;

        let id = create_project(submit_form)?;

        Ok(RefineProject::load(&id)?)
    }

    fn from_url(url: &'a str, data_format: &'a str, project_name: Option<&'a str>, record_path: Option<&'a str>) -> Result<RefineProject, Box<dyn Error>> {
        let data = utils::download(&url)?;
        let project_name = project_name.unwrap_or(&format!("{}", Utc::now())).to_string();
        let record_path = record_path.unwrap_or(&consts::RECORD_PATH.to_string()).to_string();

        let refine_data_format = format_to_refine_format(data_format.to_string());
        let mime_type = format_to_mime(data_format.to_string());

        let mut submit_form = reqwest::multipart::Form::new()
            .text("project-name", project_name.to_string())
            .text("format", refine_data_format)
            .text(
                "options",
                format!("{}\"recordPath\":{} {}", "{", record_path.clone(), "}"),
            );

        let project_data = reqwest::multipart::Part::text(data)
            .file_name(format!("data.{}", data_format))
            .mime_str(mime_type)?;

        submit_form = submit_form.part("project-file", project_data);

        let id = create_project(submit_form)?;

        Ok(RefineProject::load(&id)?)
    }

    fn from_string(data: &'a str, data_format: &'a str, project_name: Option<&'a str>, record_path: Option<&'a str>) -> Result<RefineProject, Box<dyn Error>> {
        let project_name = project_name.unwrap_or(&format!("{}", Utc::now())).to_string();
        let record_path = record_path.unwrap_or(&consts::RECORD_PATH.to_string()).to_string();

        let refine_data_format = format_to_refine_format(data_format.to_string());
        let mime_type = format_to_mime(data_format.to_string());

        let mut submit_form = reqwest::multipart::Form::new()
            .text("project-name", project_name.to_string())
            .text("format", refine_data_format)
            .text(
                "options",
                format!("{}\"recordPath\":{} {}", "{", record_path.clone(), "}"),
            );

        let project_data = reqwest::multipart::Part::text(data.to_string())
            .file_name(format!("data.{}", data_format))
            .mime_str(mime_type)?;

        submit_form = submit_form.part("project-file", project_data);

        let id = create_project(submit_form)?;

        Ok(RefineProject::load(&id)?)
    }
}

fn create_project(form: reqwest::multipart::Form) -> Result<String, Box<dyn Error>> {
    let create_project_url = format!(
        "{}{}",
        consts::REFINE_BASE_URL.to_string(), "/command/core/create-project-from-upload"
    );

    let client = reqwest::Client::new();
    let response = client.post(&create_project_url).multipart(form).send()?;
    
    if let Some(project_id) = response
        .url()
        .query_pairs()
        .filter(|(param, _value)| param == "project")
        .next()
    {
        info!("OpenRefine project created {}", project_id.1);
        Ok(project_id.1.into())
    } else {
        panic!("Failed to create OpenRefine project. No project id was returned")
    }
}

fn format_to_refine_format<'a>(format: String) -> &'a str {
    match format.as_ref() {
        "csv" => "text/line-based/csv",
        "tsv" => "text/line-based/tsv",
        "json" => "text/json",
        "xml" => "text/xml",
        _ => "",
    }
}

fn format_to_mime<'a>(mime_type: String) -> &'a str {
    match mime_type.as_ref() {
        "csv" => "text/csv",
        "tsv" => "text/tsv",
        "json" => "text/json",
        "xml" => "text/xml",
        _ => "text/plain",
    }
}
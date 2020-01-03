use chrono::prelude::*;
use dotenv::dotenv;

use crate::utils::save;

use std::env;
use std::error::Error;
use std::fs;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RefineInit<'a> {
    pub refine_script: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub data_format: &'a str,
    pub record_path: &'a str,
}

impl<'a> RefineInit<'a> {
    pub fn new(
        data_format: &'a str,
        record_path: &'a str,
        refine_script: Option<String>,
    ) -> RefineInit<'a> {
        let refine = RefineInit {
            refine_script: refine_script,
            record_path: record_path,
            data_format: data_format,
            project_id: None,
            project_name: None,
        };

        refine
    }

    pub fn create_project(
        &'a mut self,
        data: String,
        project_name: &'a str,
    ) -> Result<RefineProject, Box<dyn Error>> {
        self.project_name = Some(format!("{} [{}]", Utc::now(), project_name));

        info!("Creating an OpenRefin project");
        let mut form = reqwest::multipart::Form::new()
            .text("project-name", self.project_name.clone().unwrap())
            .text("format", self.data_format.to_string())
            .text(
                "options",
                format!("{}\"recordPath\":{} {}", "{", self.record_path.clone(), "}"),
            );

        let project_data = reqwest::multipart::Part::text(data)
            .file_name("data.json")
            .mime_str("text/json")?;

        form = form.part("project-file", project_data);

        dotenv().ok();
        let refine_base_url = env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
        let create_project_url = format!(
            "{}{}",
            refine_base_url, "/command/core/create-project-from-upload"
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
            self.project_id = Some(project_id.1.into());
        }

        Ok(RefineProject {
            project_id: self.project_id.clone().expect("Expect a project ID"),
            project_name: self.project_name.clone().expect("Expect a project name"),
            refine_script: self.refine_script.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RefineProject {
    pub refine_script: Option<String>,
    pub project_id: String,
    pub project_name: String,
}

impl RefineProject {
    pub fn open(id: &str) ->  Result<RefineProject, Box<dyn Error>> {
        info!("opening refine project {}", id);
        let refine_base_url = env::var("OPEN_REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
        let command_url = format!("{}/command/core/get-all-project-metadata", refine_base_url);

        let response = reqwest::get(&command_url)?.text()?;
        let v: Value = serde_json::from_str(&response)?;

        let project_name = v["projects"][id]["name"].as_str().unwrap();

        Ok(RefineProject {
            refine_script: None,
            project_id: id.into(),
            project_name: project_name.into(),
        })
    }

    pub fn apply_operations(&mut self) -> Result<RefineProject, Box<dyn Error>> {
        dotenv().ok();

        let project_id = self.project_id.clone();

        if let Some(script) = self.refine_script.clone() {
            info!(
                "Applying script {} to OpenRefine project {}",
                script, project_id
            );

            let script_contents = fs::read_to_string(script)?;
            let params = [("project", &project_id), ("operations", &script_contents)];
            let refine_base_url =
                env::var("OPEN_REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
            let apply_operations_url =
                format!("{}{}", refine_base_url, "/command/core/apply-operations");

            let client = reqwest::Client::new();
            client.post(&apply_operations_url).form(&params).send()?;

            info!("Script applied");
        }

        Ok(self.clone())
    }

    pub fn export(
        &mut self,
        format: Option<String>,
    ) -> Result<RefineProject, Box<dyn Error>> {
        match format {
            Some(format) => {
                info!("exporting data");
                let refine_base_url =
                    env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
                let create_project_url =
                    format!("{}{}", refine_base_url, "/command/core/export-rows");

                use std::collections::HashMap;

                let mut params = HashMap::new();
                params.insert("project", self.project_id.clone());
                params.insert("engine", r#"'{"facets": [], "mode": "row-based"}'"#.into());
                params.insert("format", format.clone());

                let client = reqwest::Client::new();
                let mut response = client.post(&create_project_url).form(&params).send()?;

                save(&mut response, &self.project_name.clone(), &format)?;
            }
            None => (),
        };

        Ok(self.clone())
    }

    pub fn print(
        &mut self,
        format: Option<String>,
    ) -> Result<RefineProject, Box<dyn Error>> {
        match format {
            Some(format) => {
                info!("exporting data");
                let refine_base_url =
                    env::var("REFINE_URL").unwrap_or("http://127.0.0.1:3333".into());
                let create_project_url =
                    format!("{}{}", refine_base_url, "/command/core/export-rows");

                use std::collections::HashMap;

                let mut params = HashMap::new();
                params.insert("project", self.project_id.clone());
                params.insert("engine", r#"'{"facets": [], "mode": "row-based"}'"#.into());
                params.insert("format", format.clone());

                let client = reqwest::Client::new();
                let mut response = client.post(&create_project_url).form(&params).send()?;

                println!("{}", response.text()?);
            },
            None => (),
        };

        Ok(self.clone())
    }
}

use crate::refine::RefineProject;
use std::error::Error;
use crate::utils::save;
use std::collections::HashMap;
use crate::consts;
use std::path::Path;

/// This trait allows you to export data from an OpenRefine project
pub trait Export {
    ///Save an OpenRefine project to disk
    fn save(&mut self, format: String, file_name: Option<String>) -> Result<String, Box<dyn Error>>;
    ///Print OpenRefine data to STDOUT
    fn print(&mut self, format: String) -> Result<(), Box<dyn Error>>;
}

impl Export for RefineProject {
    fn save(&mut self, format: String, file_name: Option<String>) -> Result<String, Box<dyn Error>> {
        let file_name = if let Some(file_name) = file_name {
            file_name
        } else {
            format!("{}.{}", self.project_name.clone(), format)
        };

        info!("saving data to disk");
        let mut data = download_data(&self.project_id, &format)?;
        save(&mut data, &file_name)?;

        let path = Path::new(&file_name).canonicalize()?.to_string_lossy().to_string();

        info!("data saved to {}", path);

        Ok(path)
    }

    fn print(&mut self, format: String) -> Result<(), Box<dyn Error>> {
        let mut data = download_data(&self.project_id, &format)?;
        
        info!("printing data:");
        println!("{}", data.text()?);

        info!("data printed");

        Ok(())
    }
}

fn download_data(project_id: &str, format: &str) -> Result<reqwest::Response, Box<dyn Error>> {
    info!("downloading data");
    let mut request_params = HashMap::new();
    request_params.insert("project",project_id);
    request_params.insert("engine", r#"'{"facets": [], "mode": "row-based"}'"#.into());
    request_params.insert("format", format);

    let download_url = format!("{}{}", consts::REFINE_BASE_URL.to_string(), "/command/core/export-rows");

    let client = reqwest::Client::new();
    let response = client.post(&download_url).form(&request_params).send()?;

    info!("data downloaded");

    Ok(response)
}
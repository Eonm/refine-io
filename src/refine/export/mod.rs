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
    fn print(&mut self, format: String) -> Result<String, Box<dyn Error>>;
}

impl Export for RefineProject {
    fn save(&mut self, format: String, file_name: Option<String>) -> Result<String, Box<dyn Error>> {
        let file_name = if let Some(file_name) = file_name {
            file_name
        } else {
            if cfg!(windows) {
                format!("{}.{}", self.project_name.clone().replace(":", "-").replace(".", "_").replace(" ", "_"), format)
            } else  {
                self.project_name.clone()
            }
        };
        
        info!("saving data to disk");
        let mut data = download_data(&self.project_id, &format)?;
        save(&mut data, &file_name)?;

        let path = Path::new(&file_name).canonicalize()?.to_string_lossy().to_string();

        info!("data saved to {}", path);

        Ok(path)
    }

    fn print(&mut self, format: String) -> Result<String, Box<dyn Error>> {
        let mut data = download_data(&self.project_id, &format)?;
        let text = data.text()?;
        
        info!("printing data:");
        println!("{}", text);

        info!("data printed");

        Ok(text)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RefineInit;
    use crate::refine::delete::Delete;

    #[test]
    fn test_save() {
        use tempfile::Builder;
        let tmp_dir = Builder::new().prefix("example").tempdir().unwrap();

        let output_file = format!("{}test.csv", tmp_dir.path().display());
        
        let mut project = RefineInit::create_project(Some("playground/input.json"), Some("json"), None, Some(r#"["_"]"#)).expect("Failed to create project");
        project.save("csv".into(), Some(output_file.clone())).expect("Failed to save");

        assert_eq!(Path::new(&output_file).exists(), true);

        project.delete().expect("Failed to delete project");
    }

    #[test]
    fn test_print() {       
        let mut project = RefineInit::create_project(Some("./playground/input.json"), Some("json"), None, Some(r#"["_"]"#)).expect("Failed to create project");
        let data = project.print("csv".into()).expect("Failed to save");
        assert_eq!(data.is_empty(), false);

        project.delete().expect("Failed to delete project");
    }
}
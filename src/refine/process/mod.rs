use crate::RefineProject;
use std::error::Error;
use crate::consts;
use crate::utils;

/// Apply operations on OpenRefine data
pub trait Process {
    fn apply_operations(&mut self) -> Result<RefineProject, Box<dyn Error>>;
}

impl Process for RefineProject {
    /// Apply a script on OpenRefine data
    fn apply_operations(&mut self) -> Result<RefineProject, Box<dyn Error>> {
        if let Some(script) = self.refine_script.clone() {
            apply_script(&self.project_id, &load_script(&script)?)?;
        }

        Ok(self.clone())
    }
}

fn apply_script(project_id: &str, script_content: &str) -> Result<(), Box<dyn Error>> {
    info!("applying script to OpenRefine project {}", project_id);

    let apply_operations_url = format!("{}{}", consts::REFINE_BASE_URL.to_string(), "/command/core/apply-operations");
    let request_params = [("project", &project_id), ("operations", &script_content)];

    let client = reqwest::Client::new();
    client.post(&apply_operations_url).form(&request_params).send()?;

    info!("script applied");
    
    Ok(())
}

fn load_script(path: &str) -> Result<String, Box<dyn Error>> {
    info!("loading script");
    let script = utils::load(path)?;
    
    info!("script loaded");
    Ok(script)
}
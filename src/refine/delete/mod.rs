use std::error::Error;
use crate::refine::RefineProject;
use crate::consts;

pub trait Delete {
    fn delete(&self) -> Result<(), Box<dyn Error>>;
}

impl Delete for RefineProject {
    fn delete(&self) -> Result<(), Box<dyn Error>> {
        info!("deleting OpenRefine project {}", self.project_id);
        delete_request(&self.project_id)?;
        info!("OpenRefine project deleted");
        Ok(())
    }
}

fn delete_request(project_id: &str) -> Result<(), Box<dyn Error>> {
    let download_url = format!("{}{}", consts::REFINE_BASE_URL.to_string(), "/command/core/delete-project");

    let request_params = [("project", &project_id)];

    let client = reqwest::Client::new();
    client.post(&download_url).form(&request_params).send()?;

    Ok(())
}
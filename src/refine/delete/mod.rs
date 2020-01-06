use std::error::Error;
use crate::refine::RefineProject;
use crate::consts;
use serde_json::Value;

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
    let mut response = client.post(&download_url).form(&request_params).send()?;
    let json_response : Value = response.json()?;

    match json_response["code"].as_str() {
        Some(code) => {
            if code == "ok" {
                Ok(())
            } else {
                error!("Failed to delete OpenRefine project {}", project_id);
                panic!("Failed to delete OpenRefine project {}", project_id);
            }
        },
        None => {
            error!("Failed to delete OpenRefine project {}", project_id);
            panic!("Failed to delete OpenRefine project {}", project_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RefineInit;

    #[test]
    #[should_panic]
    fn test_delete_project() {
        let project = RefineInit::create_project(Some("playground/input.json"), Some("json"), None, Some(r#"["_"]"#)).expect("Failed to create project");
        let project_id = project.project_id.clone();

        RefineProject::load(&project_id).expect("should not panic");
        project.delete().expect("Failed to delete project");
        RefineProject::load(&project_id).expect("should panic");
    }
}
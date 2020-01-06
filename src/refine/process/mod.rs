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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::refine::RefineInit;
    use crate::refine::delete::Delete;
    use crate::refine::import::Import;
        use crate::refine::export::Export;

    #[test]
    fn test_apply_script() {
        let mut project = RefineInit::from_string("_ - subjectFR - subjectFR\n1", "tsv", None, None).expect("Failed to create project");
        project.refine_script = Some("./playground/refine.json".into());
        project.apply_operations().expect("Failed to apply operations");

        let data = project.print("tsv".into()).expect("Failed to get remote data");
        
        assert_eq!(data, "_ - subjectFR - subjectFR\ttest\n1\t1\n");

        project.delete().expect("Failed to delete project");
    }
}
use crate::RefineProject;
use std::error::Error;
use crate::consts;
use crate::utils;
use std::thread;
use std::time;

pub enum ProcessMod {
    Sync,
    Async
}

/// Apply operations on OpenRefine data
pub trait Process {
    fn process(&mut self, process_mod: ProcessMod) -> Result<RefineProject, Box<dyn Error>>;
    fn apply_operations(&mut self) -> Result<RefineProject, Box<dyn Error>>;
    ///Wait for operations to finish
    fn apply_operations_sync(&mut self) -> Result<RefineProject, Box<dyn Error>>;
    fn get_async_processes(&mut self) -> Result<Option<String>, Box<dyn Error>>;
}

impl Process for RefineProject {

    /// Apply a script synchronously or asynchronously on OpenRefine data
    fn process(&mut self, process_mod: ProcessMod) -> Result<RefineProject, Box<dyn Error>> {
        info!("applying script to OpenRefine project {}", &self.project_id);
        
        let result = match process_mod {
            ProcessMod::Sync => self.apply_operations_sync(),
            ProcessMod::Async => self.apply_operations(),
        };

        info!("script applied");

        result
    }

    /// Apply a script on OpenRefine data
    fn apply_operations(&mut self) -> Result<RefineProject, Box<dyn Error>> {
        if let Some(script) = self.refine_script.clone() {
            apply_script(&self.project_id, &load_script(&script)?)?;
        }

        Ok(self.clone())
    }

    /// Apply a script synchronously on OpenRefine data
    fn apply_operations_sync(&mut self) ->  Result<RefineProject, Box<dyn Error>> {
        self.apply_operations()?;
        let self_copy = self.clone();

        let th = thread::spawn(move || {
            while let Some(_process) = self_copy.clone().get_async_processes().expect("Failed to get async processes") {
                thread::sleep(time::Duration::from_millis(consts::CHECK_ASYNC_INTERVAL.clone() as u64));
            };
        });

        th.join().expect("Failed to join thread");

        Ok(self.clone())
    }

    /// List async processes for a project
    fn get_async_processes(&mut self) -> Result<Option<String>, Box<dyn Error>> {
        let create_project_url = format!(
            "{}{}?project={}",
            consts::REFINE_BASE_URL.to_string(), "/command/core/get-processes", self.project_id.clone()
        );

        let client = reqwest::Client::new();
        let mut response = client.get(&create_project_url).send()?;

        let processes : serde_json::Value = response.json()?;

        if processes["processes"].as_array().expect("Failed to get json object containing processes").is_empty() {
            Ok(None)
        } else {
            Ok(Some(processes.to_string()))
        }
    }
}

fn apply_script(project_id: &str, script_content: &str) -> Result<(), Box<dyn Error>> {
    let apply_operations_url = format!("{}{}", consts::REFINE_BASE_URL.to_string(), "/command/core/apply-operations");
    let request_params = [("project", &project_id), ("operations", &script_content)];

    let client = reqwest::Client::new();
    client.post(&apply_operations_url).form(&request_params).send()?;
    
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

    #[test]
    fn test_async_process() {
        let mut project = RefineInit::from_string("_ - subjectFR - subjectFR\n1", "tsv", None, None).expect("Failed to create project");
        project.refine_script = Some("./playground/async_processes.json".into());
        project.process(ProcessMod::Sync).expect("Failed to apply operations");

        let data = project.print("tsv".into()).expect("Failed to get remote data");
        
        assert_eq!(data, "_ - subjectFR - subjectFR\techo-1\n1\t1\n");

        project.delete().expect("Failed to delete project");
    }
    
    #[test]
    fn test_get_async_process() {
        let mut project = RefineInit::from_string("_ - subjectFR - subjectFR\n1", "tsv", None, None).expect("Failed to create project");
        project.refine_script = Some("./playground/async_processes.json".into());
        project.process(ProcessMod::Sync).expect("Failed to apply operations"); 
        project.get_async_processes().expect("Failed to get async process"); 
        project.delete().expect("Failed to delete project");
    }
}
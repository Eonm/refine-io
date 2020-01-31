#[macro_use]
extern crate log;
extern crate clap;
extern crate open;
extern crate serde_json;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;

use env_logger::Env;
use std::error::Error;

mod consts;
mod utils;
mod cli;
use cli::cli;
mod refine;
use refine::{RefineInit, RefineProject, export::Export, process::Process, delete::Delete, process::ProcessMod};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli();

    if !matches.is_present("silent") {
        env_logger::from_env(Env::default().default_filter_or("info")).init();
    };

    if cfg!(windows) {
        info!("ready to refine");
    } else {
        info!("🏭 ready to refine 🛢️");
    }

    let project = import_or_open(&matches)?;
    modify(&matches, project.clone())?;
    export(&matches, project.clone())?;
    clean(&matches, project.clone())?;

    Ok(())
}

fn import_or_open(matches: &clap::ArgMatches) -> Result<RefineProject, Box<dyn Error>> {
    let project = match matches.value_of("project_id") {
        Some(id) => {
            RefineProject::load(id)?
        },
        None => { 
            RefineInit::create_project(matches.value_of("input"), matches.value_of("format"), matches.value_of("project_name"), matches.value_of("record_path"))?          
        }
    };

    if matches.is_present("open_project") {
        info!("opening OpenRefine project");

        if open::that(format!(
            "{}/project?project={}",
            consts::REFINE_BASE_URL.to_string(),
            project.project_id.clone()
        ))
        .is_ok()
        {
            info!("OpenRefine project opened");
        } else {
            warn!("failed to open OpenRefine project")
        }
    }

    Ok(project)
}

fn modify(matches: &clap::ArgMatches, mut project: RefineProject) -> Result<(), Box<dyn Error>> {
    project.refine_script =  matches.value_of("script").map(String::from);
    let process_mod = if matches.is_present("sync") {
        ProcessMod::Sync
    } else {
        ProcessMod::Async
    };
    
    if matches.is_present("script") {
        project.clone().process(process_mod)?;
    }

    Ok(())
}

fn export(matches: &clap::ArgMatches, project: RefineProject) -> Result<(), Box<dyn Error>> {
    if let Some(format) = matches.value_of("export") {
        let output = matches.value_of("output").map(String::from);
        let file_path = project.clone().save(format.into(), output)?;

        if matches.is_present("open_export") {
            info!("opening exported data");
            if open::that(file_path).is_ok() {
                    info!("data file opened");
            } else {
                warn!("failed to data file")
            }
        }
    }

    if let Some(format) = matches.value_of("print") {
        project.clone().print(format.into())?;
    }

    Ok(())
}

fn clean(matches: &clap::ArgMatches, project: RefineProject) -> Result<(), Box<dyn Error>> {
    if matches.is_present("clean") {
        project.clone().delete()?;
    }

    Ok(())
}
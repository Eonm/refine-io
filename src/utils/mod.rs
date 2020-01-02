use std::error::Error;
use std::io::copy;
use std::fs::File;
use std::fs;

pub fn download(url: &str) -> Result<String, Box<dyn Error>> {
    info!("downloading data");
    let data = reqwest::get(url)?
        .text()?;
    
    info!("data downloaded :\n\t{}", data);
    Ok(data)
}

pub fn load(path: &str) -> Result<String, Box<dyn Error>> {
    info!("loading data from {}", path);
    let data = fs::read_to_string(path)?;
    info!("data loaded :\n\t{}", data);
    Ok(data)
}

pub fn save(response: &mut reqwest::Response, path: &str, format: &str)  -> Result<(), Box<dyn Error>> {
    let mut dest = {
        let fname = format!("{}.{}", path, format);
                    
            info!("data will be located under: {}", fname);
            File::create(fname)?
        };

        copy(response, &mut dest)?;
        info!("data exported");
    Ok(())
}
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::copy;

pub fn download(url: &str) -> Result<String, Box<dyn Error>> {
    info!("downloading data");
    let data = reqwest::get(url)?.text()?;

    info!("data downloaded :\n\t{}", data);
    Ok(data)
}

pub fn load(path: &str) -> Result<String, Box<dyn Error>> {
    info!("loading data from {}", path);
    let data = fs::read_to_string(path)?;
    info!("data loaded :\n\t{}", data);
    Ok(data)
}

pub fn save(
    response: &mut reqwest::Response,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut dest = {
        let fname = format!("{}", path);
        File::create(fname)?
    };

    copy(response, &mut dest)?;
    Ok(())
}

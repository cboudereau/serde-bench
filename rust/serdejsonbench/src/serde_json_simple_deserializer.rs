use std::io::Read;
use std::{error::Error, fs::File, io::BufReader, path::Path};

use crate::Json;

pub fn read_from_file2<P: AsRef<Path>>(path: P) -> Result<Vec<Json>, Box<dyn Error>> {
    let file = File::open(path)?;
    //https://github.com/serde-rs/json/issues/160
    let size = file.metadata().unwrap().len();
    let mut bytes = Vec::with_capacity(size.try_into().unwrap());
    file.read_to_end(&mut bytes).unwrap();

    Ok(serde_json::from_slice(&bytes)?)
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Json>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

pub fn readone_from_file<P: AsRef<Path>>(path: P) -> Result<Json, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

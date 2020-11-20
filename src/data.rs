use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::{fs, io};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

const DATA_FILE: &str = "data.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Data {
    pub tasks: HashMap<String, i64>,
}

pub fn load() -> Result<Data, io::Error> {
    let file = match data_file() {
        Some(file) => file,
        None => return Ok(Data::default()),
    };

    match fs::read_to_string(file) {
        Ok(data) => Ok(serde_json::from_str(&data)?),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(Data::default()),
            kind => Err(Error::from(kind)),
        },
    }
}

pub fn store(data: Data) -> Result<(), io::Error> {
    let file = data_file().ok_or(Error::new(ErrorKind::NotFound, "No data directory."))?;

    // Create parent directory if it does not exist, yet
    if let Some(parent) = file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let vec = serde_json::to_vec_pretty(&data).unwrap();
    let mut file = File::create(file)?;
    file.write_all(&vec)?;
    file.flush()?;
    Ok(())
}

fn data_file() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("de", "s1ck", env!("CARGO_PKG_NAME"))?;
    let mut file = dirs.data_dir().to_path_buf();
    file.push(DATA_FILE);
    Some(file)
}

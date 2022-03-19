const APPLICATION_NAME: &str = "pocket_news";

use serde_derive::{Deserialize, Serialize};

use std::{io::prelude::*, path::PathBuf};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub url: String,
    pub tag: String,
    pub r#type: String,
    pub no_articles: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub consumer_key: Option<String>,
    pub source: Vec<Source>
}

#[derive(Serialize, Deserialize)]
pub struct DataFile {
    pub access_token: Option<String>
}


impl Config {
    pub fn read(filename: &str) -> Result<Self, String> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(APPLICATION_NAME).unwrap();
        let config_path = xdg_dirs.find_config_file(filename)
            .expect("Config file does not exist");

        let mut buffer = String::new();
        let datafile = fetch_toml(&config_path, &mut buffer)?;

        Ok(datafile)
    }

    pub fn store(&self, filename: &str) -> Result<(), String>{
        let xdg_dirs = xdg::BaseDirectories::with_prefix(APPLICATION_NAME).unwrap();
        let config_path = xdg_dirs.find_config_file(filename)
            .expect("Config file does not exist");

        let contents = toml::to_string(&self).unwrap();

        let mut file = File::open(config_path).unwrap();

        file.write(&contents.into_bytes()).map_err(|err| err.to_string())?;

        Ok(())
    }
}


impl DataFile {
    pub fn read(filename: &str) -> Option<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(APPLICATION_NAME).unwrap();
        let config_path = xdg_dirs.find_data_file(filename)?;

        let mut buffer = String::new();
        let datafile = fetch_toml(&config_path, &mut buffer).ok();

        datafile
    }

    pub fn store(&self, filename: &str) -> Result<(), String>{
        let xdg_dirs = xdg::BaseDirectories::with_prefix(APPLICATION_NAME).unwrap();
        let config_path = match xdg_dirs.find_data_file(filename.clone()) {
            Some(v) => v,
            None => {
                xdg_dirs.place_data_file(filename)
                    .expect("Datafile does not exist and cannot be created")
            }
        };

        let contents = toml::to_string(&self).unwrap();
        let mut file = match File::open(config_path.clone()) {
            Ok(v) => v,
            Err(_e) => {
                File::create(config_path)
                    .expect("Datafile does not exist and cannot be created")
            }
        };

        file.write(&contents.into_bytes()).unwrap();

        Ok(())
    }

    pub fn store_access_token(filename: &str, access_token: &str) -> Result<(), String>{
        let mut datafile = match Self::read(filename.clone()) {
            Some(v) => v,
            None => Self {
                access_token: None
            }
        };

        datafile.access_token = Some(access_token.to_string());
        datafile.store(filename)?;

        Ok(())
    }
}

fn fetch_toml<'a, T: serde::Deserialize<'a>>(path: &PathBuf, buffer: &'a mut String) -> Result<T, String> {
    let mut file = File::open(path).unwrap();
    file.read_to_string(buffer).unwrap();

    let config: T = toml::from_str(buffer.as_str()).unwrap();

    return Ok(config)
}


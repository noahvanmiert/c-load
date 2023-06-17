/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/

use serde::{Deserialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;


#[derive(Debug, Deserialize)]
pub struct ConfigData {
    #[serde(default)]
    pub c_flags: Vec<String>,
    #[serde(default)]
    pub verbose: bool,
}


impl Default for ConfigData {
    fn default() -> ConfigData {
        return ConfigData {
            c_flags: vec![],
            verbose: false,
        }
    }
}


pub fn config_exists() -> bool {
    return Path::new(".clconfig").exists();
}


pub fn get_config(path: &str) -> ConfigData {
    let mut file = File::open(path)
                        .expect("Failed to open config file");

    let mut file_contents = String::new(); 
    file.read_to_string(&mut file_contents)
        .expect("Failed t read config file");

    let data: ConfigData = serde_json::from_str(&file_contents)
                                      .expect("Failed to deserialize JSON");

    return data;
}
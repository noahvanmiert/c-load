/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/

use serde::{Deserialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;


/// Contains de data defined in the configuration file
/// You can create a config file by just creating `.clconfig` in the root folder
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub c_flags: Vec<String>,
    #[serde(default)]
    pub verbose: bool,
}


impl Default for Config {

    fn default() -> Config {
        return Config {
            c_flags: vec![],
            verbose: false,
        }
    }
    
}


impl Config {

    /// Returns true if the config file exists
    pub fn exists() -> bool {
        return Path::new(".clconfig").exists();
    }


    /// Returns a Config struct instance with the data loaded from `.clconfig`
    pub fn load() -> Config {
        let mut file = File::open(".clconfig")
                        .expect("Failed to open config file");

        let mut file_contents = String::new(); 

        file.read_to_string(&mut file_contents)
            .expect("Failed t read config file");

        let data: Config = serde_json::from_str(&file_contents)
                                        .expect("Failed to deserialize JSON");

        return data;
    }

}
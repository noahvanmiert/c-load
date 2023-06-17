/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;


/// Contains de data defined in the configuration file
/// You can create a config file by just creating `.clconfig` in the root folder
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_compiler")]
    pub compiler: String,

    #[serde(default = "default_output")]
    pub output: String,

    #[serde(default)]
    pub c_flags: Vec<String>,

    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub ignore: Vec<String>,
}


/// Returns default value of compiler in `.clconfig`
fn default_compiler() -> String {

    return "clang".to_string();

}


/// Returns default value of output in `.clconfig`
fn default_output() -> String {

    return "main.out".to_string();

}


impl Default for Config {

    fn default() -> Config {

        Config {
            compiler: "clang".to_string(),
            output: "main.out".to_string(),
            c_flags: vec![],
            verbose: true,
            ignore: vec![],
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
            .expect("Failed to read config file");

        let data: Config = serde_json::from_str(&file_contents)
                                      .expect("Failed to deserialize JSON");

        return data;
        
    }


    // Validates the configuration, like if the compiler is 'gcc' or 'clang'
    pub fn validate(&self) {

        if self.compiler != "gcc" && self.compiler != "clang" {
            println!("Error: unkown compiler set in `.clconfig`: {}", self.compiler);
            exit(1);
        }

    }

}
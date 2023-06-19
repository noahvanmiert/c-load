/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;


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

    #[serde(default = "default_entry")]
    pub entry: String,

    #[serde(default = "default_git")]
    pub git: bool,
}


/// Returns default value of `compiler` in `.clconfig`
/// By default `compiler` is set to `clang`
fn default_compiler() -> String {

    return "clang".to_string();

}


/// Returns default value of `output` in `.clconfig`
/// By default `output` is set to `main.out`
fn default_output() -> String {

    return "main.out".to_string();

}


/// Returns default value of `entry` in `.clconfig`
/// By default `entry` is set to `main.c`
fn default_entry() -> String {

    return "main.c".to_string();

}


/// Returns default value of `git` in `.clconfig`
/// By default `git` is set to `true`.
fn default_git() -> bool {

    return true;

}


impl Default for Config {

    /// Default trait for `Config`
    fn default() -> Config {

        Config {
            compiler: "clang".to_string(),
            output:   "main.out".to_string(),
            c_flags:  vec![],
            verbose:  true,
            ignore:   vec![],
            entry:    "main.c".to_string(),
            git:      true,
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
            .unwrap_or_else(|_| panic!("Failed to open config file"));

        let mut file_contents = String::new(); 
        file.read_to_string(&mut file_contents)
            .unwrap_or_else(|_| panic!("Failed to read config file"));

        let data: Config = serde_json::from_str(&file_contents)
            .unwrap_or_else(|_| panic!("Failed to deserialize JSON"));

        data
    }


    // Validates the configuration, like if the compiler is 'gcc' or 'clang'
    pub fn validate(&self) {

        // For now we only support `gcc` and `clang`
        if self.compiler != "gcc" && self.compiler != "clang" {
            println!("Error: unkown compiler set in `.clconfig`: {}", self.compiler);
            std::process::exit(1);
        }

        // The entry point should always be a C file because the `main` function is defined here
        if !self.entry.ends_with(".c") {
            println!("Error: entry point option in `.clconfig` should always be a `.c` file");
            std::process::exit(1);
        }

    }

}
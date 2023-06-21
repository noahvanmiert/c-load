/*
    Made by Noah Van Miert
    19/06/2023

    This file is part of the C-load project.
*/

use std::process::Command;
use crossterm::style::Color;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, ErrorKind};
use std::path::Path;

use crate::config::Config;
use crate::output::Output;


#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub headers: Vec<String>,
    pub sources: Vec<String>
}


impl Package {

    /// Returns a Package struct instance with the data loaded from `.clpackage`
    pub fn load(path: &str) -> Package {
        Self::validate(path);

        let mut file = File::open(&(path.to_string() + "/.clpackage"))
            .unwrap_or_else(|_| panic!("Failed to open package file"));

        let mut file_contents = String::new(); 
        file.read_to_string(&mut file_contents)
            .unwrap_or_else(|_| panic!("Failed to read package file"));

        let data: Package = serde_json::from_str(&file_contents)
            .unwrap_or_else(|_| panic!("Failed to deserialize JSON"));

        data
    }


    /// Loads all the packages
    pub fn load_all(clconfig: &Config) {
        for package in &clconfig.packages {
            Self::validate(&package);

            let pkg = Self::load(&package);
            pkg.compile(&package);
            pkg.get_headers(&package)
        }
    }


    /// Compiles the packages's C files into the obj files.
    pub fn compile(&self, path: &str) {  
        Self::create_dir("packages/objs");

        for (index, source) in self.sources.iter().enumerate() {
            let output_file = format!("packages/objs/{}.{}.package.obj", self.name, index);

            let mut child = Command::new("clang")
            .args(["-c", &format!("{}/{}", path, source)])
            .arg("-o")
            .arg(output_file)
            .spawn()
            .expect("Failed to create object file from sources in package");

            let status = child.wait().expect("Failed to create object file from sources in package");
        
            if !status.success() {
                eprintln!("Failed to compile source: {}", source);
                std::process::exit(1);
            }
        }
    }

    
    /// Gets the package's header files.
    pub fn get_headers(&self, path: &str) {
        let package_dir = format!("packages/{}", self.name);
        Self::create_dir(&format!("packages/{}", self.name));

        for header in &self.headers {
            if let Some(file_name) = Self::get_file_name(header) {
                let destination = format!("{}/{}", package_dir, file_name);

                if let Err(err) = Self::copy_file(&format!("{}/{}", path, header), &destination) {
                    eprintln!("Error copying header file: {}. Reason: {}", header, err);
                }
            } else {
                eprintln!("Invalid header path: {}", header);
            }
        }
    }


    /// Checks of the package exists
    fn validate(path: &str) {
        if !Path::new(&format!("{}/.clpackage", path)).exists() {
            Output::print(&format!("Error: package doesn't exist or has no `.clpackage` file: {}\n", path), Color::Red)
                .unwrap();

            std::process::exit(1);
        }
    }


    /// Returns the filename of a file
    fn get_file_name(file: &str) -> Option<String> {
        let file_name = Path::new(file).file_name()?.to_str()?;
        Some(file_name.to_owned())
    }


    /// Copies a file form source to destination
    fn copy_file(source: &str, destination: &str) -> Result<(), std::io::Error> {
        std::fs::copy(source, destination)?;
        File::open(destination)?;
        Ok(())
    }


    /// Creates a directory, if the directory already exists, do nothing.
    pub fn create_dir(path: &str) {
        match std::fs::create_dir(path) {
            Ok(_) => (),
            Err(e) => {
                match e.kind() {
                    ErrorKind::AlreadyExists => {}

                    other_e => {
                        Output::print(&format!("Failed to create `{}` folder: {}", path, other_e), Color::Red)
                            .unwrap();
                        std::process::exit(1);
                    }
                }
            }
        }
    }

}
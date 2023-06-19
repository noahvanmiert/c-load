/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/


use std::fs;
use std::fs::File;
use std::io::{Write, ErrorKind};
use std::process::{exit, Command, Stdio};
use walkdir::WalkDir;
use crossterm::style::Color;

use crate::config::Config;
use crate::output::Output;


pub struct ClCommand;

impl ClCommand {

    /// This function initializes the C project.
    pub fn init(clconfig: &Config) {

        Self::create_project_dirs();
        
        Self::create_entry_point(&clconfig);

        Self::create_gitignore();

        // Checks if `git` is enabled, if so then initialize the git repo
        Self::init_git(&clconfig);
        
        Output::print("Succesfully initialized c-load project!\n", Color::Green).unwrap();
    }


    /// Returns a list of all source files
    fn get_src_files(clconfig: &Config) -> Vec<String> {
        let mut sources: Vec<String> = Vec::new();

        /* 
        Go trough all the C source files and add them into a 
        vector so they all get compiled 
        */
        for entry in WalkDir::new("src/") {
            if let Ok(entry) = entry {
                let entry_str = entry.path().to_str().unwrap().to_string();

                if entry_str.ends_with(".c") && !clconfig.ignore.contains(&entry_str) {
                    sources.push(entry_str);
                }
            }
        }


        sources
    }


    /// This function builds the C project
    pub fn build(clconfig: &Config) {
        let sources = Self::get_src_files(clconfig);
        let output = format!("bin/{}", clconfig.output);

        if clconfig.verbose {
            Output::print("Compiling\n", Color::Green).unwrap();
            println!("->  {} {:?} -o {} {:?}\n", clconfig.compiler, sources, &output, clconfig.c_flags);
        }

        let mut child = Command::new(&clconfig.compiler)
                .args(sources)
                .arg("-o")
                .arg(&output)
                .args(&clconfig.c_flags)
                .spawn()
                .expect(&format!("Failed to start `{}`", clconfig.compiler));
        
        child.wait().expect("Compilation failed");

        if clconfig.verbose {
            Output::print("Compiliation finished succesfully!\n", Color::Green)
                   .unwrap();
        }
    }


    /// This function builds and runs the C project
    pub fn run(clconfig: &Config) {
        let output = format!("bin/{}", clconfig.output);

        if clconfig.verbose {
            Output::print("Running\n", Color::Green).unwrap();
            println!("->  ./bin/{}\n", clconfig.output);
        }

        let mut child = Command::new(format!("./{}", output))
            .spawn()
            .expect(&format!("Failed to run `./bin/{}`", clconfig.output));

        child.wait().unwrap();
    }


    /// This function prints a help message.
    pub fn help() {

        println!("Simple package manager for C\n");
        println!("Usage: cload [OPTIONS] [COMMAND]\n");
        println!("Options:");
        println!("  -h, --help  Prints this help message\n");
        println!("Commands:");
        println!("  init        Creates a new C project");
        println!("  build       Compiles everything into bin/main.out");
        println!("  run         Compiles everything and runs it");

    }


    /// This function checks if `git` is enabled, if so it will initialize a repo.
    fn init_git(clconfig: &Config) {

        if clconfig.git {
            let mut child = Command::new("git")
                .arg("init")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to run `git`");
            
            child.wait().expect("Failed to wait for `git` process");
        }

    }

    /// This function creates the `.gitignore` file
    fn create_gitignore() {

        let mut gitignore: File = File::create(".gitignore")
                                       .expect("Failed to create `.gitignore`");

        let template: &str = "bin/";
        
        gitignore.write_all(template.as_bytes())
                 .expect("Failed to write to `.gitignore`");

    }


    /// This function create the entry point (main.c)
    fn create_entry_point(clconfig: &Config) {

        let mut entry: File = File::create(format!("src/{}", clconfig.entry))
                                   .expect(&format!("Failed to create `src/{}`", clconfig.entry));

        let main_function_template: &str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\\n\");\n}\n";

        /* Write the main function to src/main.c */
        entry.write_all(main_function_template.as_bytes())
             .expect(&format!("Faild to write to src/{}", clconfig.entry));

    }


    // This function creates the project directories (src, bin)
    fn create_project_dirs() {
        Self::create_dir("src");
        Self::create_dir("bin");
    }

    
    /// This function create a project directory
    fn create_dir(path: &str) {
        match fs::create_dir(path) {
            Ok(_) => (),
            Err(e) => {
                match e.kind() {
                    ErrorKind::AlreadyExists => {
                        Output::print(&format!("Error: c-load already initialized in this project"), Color::Red)
                            .unwrap();
                        exit(1);
                    }

                    other_e => {
                        Output::print(&format!("Failed to create `{}` folder: {}", path, other_e), Color::Red)
                            .unwrap();
                        exit(1);
                    }
                }
            }
        }
    }
        
}
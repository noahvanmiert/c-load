/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/


use std::{fs, vec};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

use crate::config::Config;


pub struct ClCommand;

impl ClCommand {

    /// This function initializes the C project.
    pub fn init(clconfig: &Config) {

        fs::create_dir("src").expect("Failed to create `src/`");
        fs::create_dir("bin").expect("Failed to create `bin/`");

        Self::create_entry_point(&clconfig);

        Self::create_gitignore();

        // Checks if `git` is enabled, if so then initialize the git repo
        Self::init_git(&clconfig);
        
        println!("Succesfully initialized c-load project!");
    }


    /// Returns a list of all source files
    fn get_src_files(clconfig: &Config) -> Vec<String> {
        let mut sources: Vec<String> = vec![];

        /* 
        Go trough all the C source files and add them into a 
        vector so they all get compiled 
        */
        for entry in WalkDir::new("src/") {
            let entry = entry.unwrap();
            
            if entry.path().to_str().unwrap().to_string().ends_with(".c") 
                && !clconfig.ignore.contains(&entry.path().to_str().unwrap().to_string()) 
            {
                sources.push(entry.path().to_str().unwrap().to_string());
            }
        }

        return sources;
    }


    /// This function builds the C project
    pub fn build(clconfig: &Config) {
        let sources: Vec<String> = Self::get_src_files(clconfig);
        let output = format!("bin/{}", clconfig.output).to_string();

        if clconfig.verbose {
            println!("Compiling");
            println!("->  {} {:?} -o {} {:?}", clconfig.compiler, sources, &output, clconfig.c_flags);
        }

        let mut child = Command::new(&clconfig.compiler)
                .args(sources)
                .args(["-o", &output])
                .args(clconfig.c_flags.clone())
                .spawn()
                .expect(format!("Failed to start `{}`", clconfig.compiler).as_str());
        
        /* Wait untill the process is finished */
        child.wait().expect("Compilation failed");

        if clconfig.verbose {
            println!("Compiliation finished succesfully!");
        }
    }


    /// This function builds and runs the C project
    pub fn run(clconfig: &Config) {

        let output = format!("bin/{}", clconfig.output).to_string();

        if clconfig.verbose {
            println!("Running");
            println!("->  ./bin/{}\n", clconfig.output);
        }

        let mut child = Command::new(format!("./{}", output))
                                       .spawn()
                                       .expect(format!("Failed to run `./bin/{}`", clconfig.output).as_str());

        /* Wait untill the process is finished */
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
            
            child.wait().unwrap();
        }

    }


    fn create_gitignore() {

        let mut gitignore: File = File::create(".gitignore")
                                       .expect("Failed to create `.gitignore`");

        let template: &str = "bin/";
        
        gitignore.write_all(template.as_bytes())
                 .expect("Failed to write to `.gitignore`");

    }


    fn create_entry_point(clconfig: &Config) {

        let mut entry: File = File::create(format!("src/{}", clconfig.entry))
                                   .expect(format!("Failed to create `src/{}`", clconfig.entry)
                                   .as_str());

        let main_function_template: &str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\\n\");\n}\n";

        /* Write the main function to src/main.c */
        entry.write_all(main_function_template.as_bytes())
             .expect(format!("Faild to write to src/{}", clconfig.entry).as_str());

    }
        
}
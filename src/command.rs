/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/


use std::{fs, vec};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use walkdir::WalkDir;

use crate::config::Config;


pub struct ClCommand;

impl ClCommand {

    /// This function initializes the C project.
    pub fn init() {

        fs::create_dir("src").expect("Failed to create `src/`");
        fs::create_dir("bin").expect("Failed to create `bin/`");

        let mut src_file: File = File::create("src/main.c")
                                      .expect("Failed to create `src/main.c`");

        let main_function_template: &str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\\n\");\n}\n";

        /* Write the main function to src/main.c */
        src_file.write_all(main_function_template.as_bytes())
                .expect("Faild to write to src/main.c");

        let mut gitignore_file: File = File::create(".gitignore")
                                            .expect("Failed to create `.gitignore`");

        let gitignore_file_template: &str = "bin/";
        gitignore_file.write_all(gitignore_file_template.as_bytes())
                      .expect("Failed to write to `.gitignore`");

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
            println!("->  ./bin/{}", clconfig.output);
        }

        let mut child = Command::new(format!("./{}", output))
                                       .spawn()
                                       .expect(format!("Failed to run `./bin/{}`", clconfig.output).as_str());

        /* Wait untill the process is finished */
        child.wait().unwrap();

        if clconfig.verbose {
            println!("Progam ran succesfully!")
        }
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
        
}
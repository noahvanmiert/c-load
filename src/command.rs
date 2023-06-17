/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/


use std::fs;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;
use std::process::Command;

use crate::config::ConfigData;


/// This function initializes the C project.
pub fn init() {
    fs::create_dir("src").expect("Failed to create src/");
    fs::create_dir("bin").expect("Failed to create bin/");

    let mut src_file: File = File::create("src/main.c")
                                  .expect("Failed to create src/main.c");

    let entry_point_template: &str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\\n\");\n}\n";

    /* Write the main function to src/main.c */
    src_file.write_all(entry_point_template.as_bytes())
            .expect("Faild to write to src/main.c");

    let mut gitignore_file: File = File::create(".gitignore")
                                        .expect("Failed to create .gitignore");

    let gitignore_file_template: &str = "bin/";
    gitignore_file.write_all(gitignore_file_template.as_bytes())
                  .expect("Failed to write to .gitignore");
}


/// This function builds the C project
pub fn build(clconfig: &ConfigData) {
    let mut sources: Vec<String> = vec![];

    /* 
       Go trough all the C source files and add them into a 
       vector so they all get compiled 
    */
    for entry in WalkDir::new("src/") {
        let entry = entry.unwrap();
        
        if entry.path().to_str().unwrap().to_string().ends_with(".c") {
            sources.push(entry.path().to_str().unwrap().to_string());
        }
    }

    if clconfig.verbose {
        println!("gcc {:?} -o bin/main.out {:?}", sources, clconfig.c_flags);
    }

    let mut child = Command::new("gcc")
            .args(sources)
            .args(["-o", "bin/main.out"])
            .args(clconfig.c_flags.clone())
            .spawn()
            .expect("Failed to start gcc command");
    
    /* Wait untill the process is finished */
    child.wait().unwrap();
}


/// This function builds and runs the C project
pub fn run() {
    let mut child = Command::new("./bin/main.out")
                                   .spawn()
                                   .expect("Failed to run ./bin/main.out");

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

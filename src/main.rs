use std::fs;
use std::fs::File;
use std::io::Write;
use std::env;
use std::process::{exit, Command};
use walkdir::WalkDir;


fn init() {
    fs::create_dir("src").expect("Failed to create src/");
    fs::create_dir("bin").expect("Failed to create bin/");

    let mut src_file: File = File::create("src/main.c").expect("Failed to create src/main.c");

    let entry_point_template: &str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\\n\");\n}\n";

    /* Write the main function to src/main.c */
    src_file.write_all(entry_point_template.as_bytes()).expect("Faild to write to src/main.c");

    let mut gitignore_file: File = File::create(".gitignore").expect("Failed to create .gitignore");
    let gitignore_file_template: &str = "bin/";
    gitignore_file.write_all(gitignore_file_template.as_bytes()).expect("Failed to write to .gitignore");
}


fn build() {
    let mut sources: Vec<String> = vec![];

    for entry in WalkDir::new("src/") {
        let entry = entry.unwrap();
        
        if entry.path().to_str().unwrap().to_string().ends_with(".c") {
            sources.push(entry.path().to_str().unwrap().to_string());
        }
    }

    let mut child = Command::new("gcc")
            .args(sources)
            .args(["-o", "bin/main.out"])
            .spawn()
            .expect("Failed to start gcc command");
    
    child.wait().unwrap();
}


fn run() {
    let mut child = Command::new("./bin/main.out").spawn().expect("Failed to run ./bin/main.out");
    child.wait().unwrap();
}


fn help() {
    println!("Simple package manager for C\n");
    println!("Usage: cload [OPTIONS] [COMMAND]\n");
    println!("Options:");
    println!("  -h, --help  Prints this help message\n");
    println!("Commands:");
    println!("  init        Creates a new C project");
    println!("  build       Compiles everything into bin/main.out");
    println!("  run         Compiles everything and runs it");
}


fn parse_option(arg: String) {
    if arg == "-h" || arg == "--help" {
        help();
        exit(0);
    }

    println!("Unkown option: {}", arg);
    exit(1);
}


fn parse_command(command: String) {
    if command == "init" {
        init();
        return;
    }

    if command == "build" {
        build();
        return; 
    }

    if command == "run" {
        build();
        run();
        return; 
    }

    println!("Unkown command: {}", command);
    exit(1);
}


fn main() {
    if env::args().len() < 2 {
        help();
    }

    for argument in env::args().skip(1) {
        if argument.starts_with('-') {
            parse_option(argument);
            continue;
        } 

        parse_command(argument);
    }
}

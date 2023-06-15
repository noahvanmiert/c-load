use std::fs;
use std::fs::File;
use std::io::Write;
use std::env;
use std::process::exit;


fn init() {
    fs::create_dir("src-test").expect("Failed to create src/");
    fs::create_dir("bin").expect("Failed to create bin/");

    let mut src_file: File = File::create("src-test/main.c").expect("Failed to create src/main.c");

    let entry_point_template: &str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\");\n}\n";

    /* Write the main function to src/main.c */
    src_file.write_all(entry_point_template.as_bytes()).expect("Faild to write to src/main.c");

    let mut gitignore_file: File = File::create(".gitignore-test").expect("Failed to create .gitignore");
    let gitignore_file_template: &str = "bin/";
    gitignore_file.write_all(gitignore_file_template.as_bytes()).expect("Failed to write to .gitignore");
}


fn help() {
    println!("Simple package manager for C\n");
    println!("Usage: cload [OPTIONS] [COMMAND]\n");
    println!("Options:");
    println!("  -h, --help  Prints this help message\n");
    println!("Commands:");
    println!("  init        Creates a new C project");
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

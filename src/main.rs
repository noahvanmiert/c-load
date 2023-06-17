/*
    Made by Noah Van Miert
    15/06/2023

    This file is part of the C-load project.
*/


use std::env;
use std::process::exit;

mod command;


/// This function parses the given options (flags)
fn parse_option(arg: String) {
    if arg == "-h" || arg == "--help" {
        command::help();
        exit(0);
    }

    println!("Unkown option: {}", arg);
    exit(1);
}


/// This function parses the given command
fn parse_command(command: String) {
    if command == "init" {
        command::init();
        return;
    }

    if command == "build" {
        command::build();
        return; 
    }

    if command == "run" {
        command::build();
        command::run();
        return; 
    }

    println!("Unkown command: {}", command);
    exit(1);
}


fn main() {
    /* Check if enough commands are given */
    if env::args().len() < 2 {
        command::help();
    }

    for argument in env::args().skip(1) {
        if argument.starts_with('-') {
            parse_option(argument);
            continue;
        } 

        parse_command(argument);
    }
}

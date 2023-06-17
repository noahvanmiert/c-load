/*
    Made by Noah Van Miert
    15/06/2023

    This file is part of the C-load project.
*/


use std::env;
use std::process::exit;

use config::Config;

mod command;
mod config;


/// This function parses the given options (flags)
fn parse_option(arg: String, _clconfig: &Config) {
    if arg == "-h" || arg == "--help" {
        command::help();
        exit(0);
    }

    println!("Unkown option: {}", arg);
    exit(1);
}


/// This function parses the given command
fn parse_command(command: String, clconfig: &Config) {
    if command == "init" {
        command::init();
        return;
    }

    if command == "build" {
        command::build(&clconfig);
        return; 
    }

    if command == "run" {
        command::build(&clconfig);
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

    let mut clconfig = Config { ..Default::default() };

    if Config::exists() {
        clconfig = Config::load();
        clconfig.validate();
    }

    for argument in env::args().skip(1) {
        if argument.starts_with('-') {
            parse_option(argument, &clconfig);
            continue;
        } 

        parse_command(argument, &clconfig);
    }
}

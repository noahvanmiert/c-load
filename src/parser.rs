/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/


use std::process::exit;

use crate::Config;
use crate::command;


pub struct Parser;

impl Parser {

    pub fn parse_args(clconfig: &Config) {

        for arg in std::env::args().skip(1) {
            if arg.starts_with('-') {
                Self::parse_option(arg, &clconfig);
                continue;
            } 
    
            Self::parse_command(arg, &clconfig);
        }

    }


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
    pub fn parse_command(command: String, clconfig: &Config) {

        match command.as_str() {

            "init" => {
                command::init();
                return;
            }

            "build" => {
                command::build(&clconfig);
                return;
            }

            "run" => {
                command::build(&clconfig);
                command::run(&clconfig);
                return;
            }

            _ => {
                println!("Unkown command: {}", command);
                exit(1);
            }

        }
        
    }

    
    /// This function checks if enough arguments are provided
    pub fn check_args_length() {

        if std::env::args().len() < 2 {
            command::help();
        }

    }

}
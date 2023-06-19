/*
    Made by Noah Van Miert
    17/06/2023

    This file is part of the C-load project.
*/

use crossterm::style::Color;

use crate::Config;
use crate::command::ClCommand;
use crate::output::Output;


pub struct Parser;

impl Parser {

    pub fn parse_args(clconfig: &Config) {

        for arg in std::env::args().skip(1) {
            if arg.starts_with('-') {
                Self::parse_option(&arg, clconfig);
            } else {
                Self::parse_command(&arg, clconfig);
            }
        }

    }


    /// This function parses the given options (flags)
    fn parse_option(option: &str, _clconfig: &Config) {

        match option {
            "-h" | "--help" => {
                ClCommand::help();
                std::process::exit(0);
            }

            _ => {
                Output::print(&format!("Unkown option: {}", option), Color::Red).unwrap();
                std::process::exit(1);
            }
        }

    }


    /// This function parses the given command
    pub fn parse_command(command: &str, clconfig: &Config) {

        match command {

            "init" => {
                ClCommand::init(&clconfig);
            }

            "build" => {
                ClCommand::build(&clconfig);
            }

            "run" => {
                ClCommand::build(&clconfig);
                ClCommand::run(&clconfig);
            }

            _ => {
                Output::print(&format!("Unkown command: {}", command), Color::Red).unwrap();
                std::process::exit(1);
            }

        }
        
    }

    
    /// This function checks if enough arguments are provided
    pub fn check_args_length() {

        if std::env::args().len() < 2 {
            ClCommand::help();
        }

    }

}
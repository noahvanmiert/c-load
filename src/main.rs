/*
    Made by Noah Van Miert
    15/06/2023

    This file is part of the C-load project.
*/


use config::Config;
use parser::Parser;

mod command;
mod config;
mod parser;
mod output;


fn main() {
    Parser::check_args_length();

    let mut clconfig = Config { ..Default::default() };

    if Config::exists() {
        clconfig = Config::load();
        clconfig.validate();
    }

    Parser::parse_args(&clconfig);
}

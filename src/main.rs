/*
    Made by Noah Van Miert
    15/06/2023

    This file is part of the C-load project.
*/


use config::Config;
use parser::Parser;
use package::Package;

mod command;
mod config;
mod package;
mod parser;
mod output;


fn main() {
    Parser::check_args_length();

    let mut clconfig = Config { ..Default::default() };

    if Config::exists() {
        clconfig = Config::load();
        clconfig.validate();
    }

    if !clconfig.packages.is_empty() {
        Package::create_dir("packages/");
        Package::load_all(&clconfig);
    }

    Parser::parse_args(&clconfig);
}

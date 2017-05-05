//! This library provides the functions/structs/methods used by the main binary. They are included
//! here in the hopes that they can be illuminating to users.

#[macro_use] extern crate serde_derive;
extern crate toml;
extern crate time;
extern crate core;

use std::fs::File;
use std::io::prelude::*;

pub mod types;
pub mod repo;
pub mod includes;
pub mod render;

/// Given a filepath, read the .toml file there as containing the directories/templates.
pub fn read_toml_dir(template_path: &str) -> types::Project {
    let mut template_file =
        if let Ok(f) = File::open(&template_path) {
            f
        }
        else {
            println!("File {:?} could not be opened. Check that it exists.", template_path);
            std::process::exit(0x0f00);
        };
    let mut template = String::new();
    template_file.read_to_string(&mut template)
        .expect("Template file read failed");
    toml::from_str(&template).unwrap()
}
    
/// Given a PathBuf, read the .toml file there as a configuration file.
pub fn read_toml_config(config_path: std::path::PathBuf) -> types::Config {
    let mut file =
        if let Ok(f) = File::open(&config_path) {
            f
        }
        else {
            println!("File {:?} could not be opened. Check that it exists.", config_path);
            std::process::exit(0x0f00);
        };
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)
        .expect("File $HOME/.pi.toml read failed");
    toml::from_str(&toml_str).unwrap()
}

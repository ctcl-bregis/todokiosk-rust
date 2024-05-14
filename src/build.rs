// ToDoKiosk - CTCL 2023-2024
// File: src/build.rs
// Purpose: Build needed files
// Created: March 10, 2024
// Modified: May 14, 2024

use std::fs::File;
use std::io::{Read, Error};
use std::result::Result;

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();

    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

fn check_dir(path: &str) {
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path).unwrap_or_else(|_| panic!("Could not create directory {}", &path));
    }
}

fn main() {
    let grass_options: grass::Options = grass::Options::default()
        .style(grass::OutputStyle::Compressed);

    check_dir("static/");
    
    let scss = read_file("src/common.scss").unwrap();
    let css = grass::from_string(scss, &grass_options).unwrap();
    std::fs::write("static/common.css", css).unwrap();
}
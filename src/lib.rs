// ToDoKiosk - CTCL 2023-2024
// File: src/lib.rs
// Purpose: Commonly used functions
// Created: March 11, 2024
// Modified: March 11, 2024

use std::fs::File;
use std::io::{Read, Error};

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).unwrap();
    let mut buff = String::new();

    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}

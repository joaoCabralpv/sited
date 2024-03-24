use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::process;

pub fn read(file: &str) -> String {
    if !Path::new(file).exists() {
        let result = File::create(file);
        if result.is_err() {
            eprint!("Error creating file: {:?}", result.unwrap_err());
            process::exit(2);
        }
        return String::new();
    }
    let file_read = fs::read_to_string(file);

    if file_read.is_err() {
        eprintln!("Error opening file {:?}", file_read);
        process::exit(3)
    }
    file_read.unwrap()
}

pub fn write(file_path: &str, text: &str) {
    let result = fs::write(file_path, text);
    if result.is_err() {
        eprintln!("Error writing to file");
        process::exit(4);
    }
}

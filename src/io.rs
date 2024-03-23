use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read(file:&str)->String{
    let path = Path::new(file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(e) => panic!("couldn't read {}: {}",display, e),
        _ => (),
    };
    s
}

pub fn write(file:&str,data:&str){
    let path = Path::new(file);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("coulden't create {}: {}",display, why),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}",display, why),
        Ok(_) => (),
    }
}
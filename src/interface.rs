use crossterm::{queue, style::Print};
use std::io::{stdout, Write};
//use std::process::exit;
use crate::io;

pub fn hello()
{
    let s = io::read("src/io.rs");
    io::write("test", &s)
}

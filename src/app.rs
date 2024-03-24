use crossterm::{
    event::{self, Event, KeyEvent},
    queue,
    style::Print,
};
use std::io::{stdout, Write};
//use std::process::exit;
use self::get_file::get_file;
use crate::io;
use std::collections::VecDeque;
mod input;
mod get_file;

pub struct App {
    event_queue: VecDeque<Event>,
    file_name:String,
    text:String,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        let file_name = get_file();
        App {
            event_queue: VecDeque::new(),
            text:io::read(&file_name),
            file_name:file_name,
            exit: false,
        }
    }
    //returns false when the aplication exits
    pub fn update(&mut self) -> bool {
        self.handle_input();
        self.exit
    }

    fn handle_input(&mut self) {
        input::input(&mut self.event_queue);

        loop {
            let event = self.event_queue.pop_front();
            let event = match event {
                None => break,
                Some(e) => e,
            };
            if let Event::Key(k) = event {self.handle_key(k)}
        }
    }
    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.modifiers == event::KeyModifiers::CONTROL && key.code == event::KeyCode::Char('c') {
            self.exit = true
        } // exits when ^C is pressed
    }
}

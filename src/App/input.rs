use crossterm::event::{poll, read, Event};
use std::{collections::VecDeque, time::Duration};

pub fn input(vector: &mut VecDeque<Event>) {
    if poll(Duration::from_millis(1)).expect("Error polling events") {
        let event = read();
        match event {
            Ok(e) => vector.push_back(e),
            Err(e) => panic!("Error reading input"),
        }
    }
}

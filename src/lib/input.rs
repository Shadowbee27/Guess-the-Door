use crate::menu::start_menu;
use log::{debug, error};
use std::io;

pub fn name_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    debug!("Checking for invalid chars");
    for c in input.chars() {
        if c.is_alphabetic() || c.is_numeric() {
        } else {
            error!("Invalid character found in your name");
            start_menu()
        }
    }
    if input.is_empty(){
        error!("Your name has to contain Characters");
        start_menu()
    }
    input
}
pub fn int_input() -> i64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<i64>();
        debug!("Checking if inputted Number is valid");
        match input {
            Ok(ok) => {
                if matches!(ok, 1..4) {
                    return ok;
                } else {
                    error!("Number out of range.");
                }
            }
            Err(e) => error!("Error: ({}). Please try again", e),
        }
    }
}

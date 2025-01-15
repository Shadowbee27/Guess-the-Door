use crate::menu::start_menu;
use std::io;

pub fn name_input() -> String {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        for c in input.chars() {
            if c.is_alphabetic() || c.is_numeric() {
            } else {
                println!("Invalid character found in your name");
                start_menu()
            }
        }
        return input;
    }
}
pub fn int_input() -> i64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<i64>();
        match input {
            Ok(ok) => {
                if matches!(ok, 0..4) {
                    return ok;
                } else {
                    eprintln!("Number out of range.");
                }
            }
            Err(e) => eprintln!("Error: ({}). Please try again", e),
        }
    }
}

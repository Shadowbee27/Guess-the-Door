use crate::gamefiles::game::game_loop;
use crate::lib;
use std::process::exit;
pub fn start_menu() {
    println!("Hello what is your Name?");
    let name: String = lib::input::name_input();
    menu(name)
}

pub fn menu(name: String) {
    println!("Hello {}", name);
    println!("Please choose a option: \n 1 => Start the Game \n 2 => Quit \n 3 => Change name  ");
    let doing = lib::input::int_input();
    match doing {
        2 => exit(0),
        3 => start_menu(),
        1 => game_loop(name),
        _ => {
            eprintln!("invalid_input");
            menu(name)
        }
    }
}

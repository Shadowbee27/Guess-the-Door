use crate::gamefiles::game::game_loop;
use crate::lib;
use std::process::exit;

pub fn start_menu() {
    println!("Hello what is your Name?");
    let name = lib::input::menu_input();

    println!("Hello {}", name);
    menu();
}

pub fn menu() {
    println!("Choose a option: \n s => Start the Game \n e => Change name \n q => Quit ");
    let doing = lib::input::menu_input();
    match doing.trim() {
        "q" => exit(0),
        "e" => start_menu(),
        "s" => game_loop(),
        _ => {
            println!("invalid_input");
            menu()
        }
    }
}

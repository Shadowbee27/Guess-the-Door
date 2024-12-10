use crate::gamefiles::game::game_loop;
use crate::lib;
use std::process::exit;

pub fn start() {
    println!("Hello what is your Name?");
    let name = lib::input::menu_input();

    println!("Hello {}", name);
    menu();
}

pub fn menu() {
    println!(
        "Choose a option: \n s => Start the Game \n q => Quit \n e => let somebody else play "
    );
    let doing = lib::input::menu_input();
    match doing.trim() {
        "q" => exit(0),
        "e" => start(),
        "s" => game_loop(),
        _ => {
            println!("invalid_input");
            menu()
        }
    }
}

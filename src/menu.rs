use crate::gamefiles::game::game_loop;
use crate::lib;
use std::io;
use std::process::exit;

pub fn start_menu() {
    println!("Hello what is your Name?");
    let name = lib::input::menu_input();
    println!("Hello {}\nDo you want to proceed with this name? Press enter to continue and type any letter to change the name ", name);
    let mut proceed = String::new();
    io::stdin().read_line(&mut proceed);
    if proceed.trim().is_empty() {
        menu()
    } else {
        start_menu();
    }
}

pub fn menu() {
    println!("Choose a option: \n 1 => Start the Game \n 2 => Quit \n 3 => Change name  ");
    let doing = lib::input::int_input();
    match doing {
        2 => exit(0),
        3 => start_menu(),
        1 => game_loop(),
        _ => {
            println!("invalid_input");
            menu()
        }
    }
}

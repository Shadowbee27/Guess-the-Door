use bevy::prelude::{warn, error};
use bevy::prelude::{Component};
use bevy::app::{App, Startup};
use rand::Rng;
use std::io;
use std::process::exit;
use bevy::DefaultPlugins;
use bevy::log::info;
use bevy::utils::{error, warn};

#[derive(Component)]
pub enum GameState {
    Startup,
    Menu,
    InGame,
}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, start).run();
}
fn invalid_input(function: i8) {
    println!("Invalid Input");
    warn!("Invalid Input");
    match function {
        1 => menu(),
        2 => store(),
        3 => {
            println!("Please input a valid name");
            start()
        }
        4 => {
            println!("Please input a valid door");
            game_loop(0)
        }
        _ => {
            error!("Invalid Input got called without a trace, this will result in a crash.");
            panic!("A fatal Error occurred")
        }
    }
}
fn start() {
    info!("Game starts");
    println!("Hello what is your Name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("error");
    if name.is_empty() == true {
        invalid_input(3)
    } else {
        println!("Hello {}", name);
        menu();
    }
}
fn menu() {
    info!("Menu");
    println!("Start the Game:s Quit:q let somebody else play:e ");
    let mut doing = String::new();
    io::stdin().read_line(&mut doing).expect("An fatal error");
    match doing.trim() {
        "q" => exit(0),
        "e" => start(),
        "s" => game_loop(1),
        _ => invalid_input(1),
    }
}
fn game_loop(from:i8) {
    info!("Ingame");
    let mut score: i8 = 0;
    loop {
        println!("Your score is: {}", score);
        println!("In front of you are three doors, one of them kills you.Which one do you choose? Enter a number between 1 and 3.");
        let secretnumber: i8 = rand::thread_rng().gen_range(1..=3);
        let guess = input();
        let ergebnis = vergleich(guess, secretnumber);
        if ergebnis == false {
            println!(
                "You survived, the ghost was behind the {} door",
                secretnumber
            );
            score = score + 1;
        } else if score == 32 {
            println!("You won");

        } else {
            println!("You are death");
            break;
        }
    }
    store();
}
fn input() -> i8 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i8 = guess.trim().parse().expect("Please type a number!");
    return guess;}

fn vergleich(guess: i8, secretnumber: i8) -> bool {
    if guess == secretnumber {
        true
    } else {
        false
    }
}
fn store() {
    println!("Where do you want to store your score? Server: s/ lokal: l");
    let mut storage = String::new();
    io::stdin()
        .read_line(&mut storage)
        .expect("fatal error accorded");
    match storage.trim() {
        "s" => println!("sorry but this is not avalibil"),

        "l" => println!("sorry but this is not avalibil"),
        _ => invalid_input(2),
    }
    menu()
}

use rand::Rng;
use std::fs::{read, File};
use std::process::exit;
use std::{fs, io};

fn main() {
    start();
}
fn invalid_input(function: i8) {
    println!("Invalid Input");
    match function {
        1 => menu(),
        2 => store(),
        3=> {
            println!("Please input a valid name");
            start()
        }
        _ => panic!("A fatal Error occurred"),
    }
}
fn start() {
    println!("Hello what is your Name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("error");
    println!("Hello {}", name);
    menu();
}
fn menu() {
    println!("Start the Game:s Quit:q let somebody else play:e ");
    let mut doing = String::new();
    io::stdin().read_line(&mut doing).expect("An fatal error");
    match doing.trim() {
        "q" => exit(1),
        "e" => start(),
        "s" => game_loop(),
        _ => invalid_input(1),
    }
}
fn game_loop() {
    let mut score: i8 = 0;
    loop  {
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
        }else if score == 32{
            println!("You won");
            break
        } else {
            println!("You are death");
            break
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
    return guess;
}
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

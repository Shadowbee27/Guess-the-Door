use rand::Rng;
use std::io;
use std::process::exit;

fn main() {
    game_loop();
}
fn game_loop() {
    let mut score: i8 = 0;
    while score < 32 {
        println!("Your score is: {}", score);
        println!("Infront of you are 3 door, one of them kills you. Which one are you opping enter a number between 1 and 3");
        let secretnumber: i8 = rand::thread_rng().gen_range(1..=3);
        let guess = input();
        let ergebnis = vergleich(guess, secretnumber);
        if ergebnis == false {
            println!(
                "You survived, the ghost  {}",
                secretnumber
            );
            score = score + 1;
        } else {
            println!("You are death");
            score = 0;
            let mut idk: bool = idk();
            if idk == true {
            } else {
                panic!()
            }
        }
    }
    println!("Du hast gewonnen ");
}
fn input() -> i8 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: i8 = guess.trim().parse().expect("Please type a number!");
    return guess;
}
fn compere(guess: i8, secretnumber: i8) -> bool {
    if guess == secretnumber {
        true
    } else {
        false
    }
}

fn idk() -> bool {
    println!("Do you want to play agian (y/n)");
    let mut idks = String::new();
    io::stdin().read_line(&mut idks).expect("A Error eccurt");
    match idks {
        y => return true,
        n => return false,
        _ => panic!(),
    }
}
fn quit() {
    panic!()
}

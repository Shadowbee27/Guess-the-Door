use crate::lib::input;
use crate::menu::menu;
use rand::Rng;

pub fn game_loop() {
    let score: i8 = 0;
    loop {
        println!("Your score is: {}", score);
        println!("In front of you are three doors, one of them kills you. Which one do you choose? Enter a number between 1 and 3.");
        let secretnumber: i64 = rand::rng().random_range(1..4);
        println!("Your secret number is: {}", secretnumber);
        let guess: i64 = input::int_input();
        if secretnumber == guess {
            println!("You are died \r Your score was {}", score);
            break;
        } else {
            score + 1;
            println!("The wrong door was {}", secretnumber);
        }
    }
    menu()
}

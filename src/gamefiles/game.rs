use crate::gamefiles::store;
use crate::lib::input;
use rand::Rng;
pub fn game_loop(name: String) {
    let mut score: i8 = 0;
    loop {
        println!("Your score is: {}", score);
        println!("In front of you are three doors, one of them kills you. Which one do you choose? Enter a number between 1 and 3.");
        let secret_number: i64 = rand::rng().random_range(1..4);
        let guess: i64 = input::int_input();
        if secret_number == guess {
            println!("You are died \nYour score was {}", score);
            break;
        } else if score > 32 {
            println!("You won");
            break;
        } else {
            score += 1;
            println!("The wrong door was {}", secret_number);
        }
    }
    store::store(name, score);
}

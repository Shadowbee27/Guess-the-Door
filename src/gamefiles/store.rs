use crate::lib::remove_access_scores::ScoreboardState;
use crate::lib::{check_for_scoreboard, decode, remove_access_scores};
use crate::menu;
use base64::prelude::*;
use std::fs::*;
use std::io::Write;
const SCOREBOARD_PATH: &str = "src/gamefiles/scoreboard.dat";
pub fn store(name: String, score: i8) {
    if !check_for_scoreboard::check_if_path_exist() {
        println!("Scoreboard file not found, creating it now.");
        match File::create(SCOREBOARD_PATH) {
            Ok(_) => {
                println!("created scoreboard successfully");
            }
            Err(e) => panic!("Error creating scoreboard. This is a state of no recovery, the program will crash. Exiting because: {}", e),
        }
    } else {
        println!("Found scoreboard");
    }
    let existing_scoreboard = decode::decode();
    let remove_score: (ScoreboardState, String) = remove_access_scores::remove_scores(name.clone());
    println!("return is {}", remove_score.1);
    if remove_score.0 == ScoreboardState::NoAdding {
    } else if remove_score.0 == ScoreboardState::ReplaceAdding {
        let existing_scoreboard = existing_scoreboard.replace(&remove_score.1, "");
        println!("Replaced score");
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(SCOREBOARD_PATH)
        {
            Ok(file) => file,
            Err(e) => panic!("File error. {}", e),
        };
        let add_score: String = format!("{existing_scoreboard} \n {name} has score: {score}\n");
        println!("Adding score...");
        match file.write_all(BASE64_STANDARD.encode(add_score).as_bytes()) {
            Ok(f) => {
                println!("Added score");
                f
            }
            Err(e) => panic!("Error while adding score. Exiting because: {}", e),
        }
    } else if remove_score.0 == ScoreboardState::JustAdding {
        let existing_scoreboard = decode::decode();
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(SCOREBOARD_PATH)
        {
            Ok(file) => file,
            Err(e) => panic!("File error. {}", e),
        };
        let add_score: String = format!("{existing_scoreboard} \n {name} has score: {score}\n");
        println!("Adding score...");
        match file.write_all(BASE64_STANDARD.encode(add_score).as_bytes()) {
            Ok(f) => {
                println!("Added score");
                f
            }
            Err(e) => panic!("Error while adding score. Exiting because: {}", e),
        }
    }
    menu::menu(name)
}

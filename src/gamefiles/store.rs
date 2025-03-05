use crate::lib::remove_access_scores::ScoreboardState;
use crate::lib::{check_for_scoreboard, decode, remove_access_scores};
use crate::menu;
use base64::prelude::*;
use log::{debug, warn};
use log::{error, info};
use std::fs::*;
use std::io::Write;

const SCOREBOARD_PATH: &str = "src/gamefiles/scoreboard.dat";
pub fn store(name: String, score: i8) {
    debug!("Check for scoreboard");
    if !check_for_scoreboard::check_if_path_exist() {
        warn!("Scoreboard file not found, creating it now.");
        match File::create(SCOREBOARD_PATH) {
            Ok(_) => {
                info!("created scoreboard successfully");
            }
            Err(e) => {
                error!("Error creating scoreboard. This is a state of no recovery, the program will crash. Exiting because: {}", e);
                panic!()
            }
        }
    } else {
        info!("Found scoreboard");
    }
    let existing_scoreboard = decode::decode();
    debug!("Calling remove access scores");
    let remove_score: (ScoreboardState, i8) =
        remove_access_scores::remove_scores(name.clone(), score);
    if remove_score.0 == ScoreboardState::NoAdding {
        menu::menu(name)
    } else if remove_score.0 == ScoreboardState::ReplaceAdding {
        let replacement_string: String = format!("\n {name} has score: {}", remove_score.1);
        let new_scoreboard = existing_scoreboard.replace(replacement_string.as_str(), "");
        info!("Replaced score");
        write_to_scoreboard(new_scoreboard, name, score)
    } else if remove_score.0 == ScoreboardState::JustAdding {
        write_to_scoreboard(decode::decode(), name, score)
    }
}
fn write_to_scoreboard(scoreboard: String, name: String, score: i8) {
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(SCOREBOARD_PATH)
    {
        Ok(file) => file,
        Err(e) => {
            error!("File error. {}", e);
            panic!()
        }
    };
    let add_score: String = format!("{scoreboard} \n {name} has score: {score}\n");
    debug!("Adding score...");
    match file.write_all(BASE64_STANDARD.encode(add_score).as_bytes()) {
        Ok(f) => {
            info!("Added score");
            f
        }
        Err(e) => {
            error!("Error while adding score. Exiting because: {}", e);
            panic!()
        }
    }
    menu::menu(name)
}

use crate::lib::decode;
use log::{debug, error, info};
use std::cmp::*;

#[derive(PartialEq, Debug)]
pub enum ScoreboardState {
    ReplaceAdding,
    NoAdding,
    JustAdding,
}
pub fn remove_scores(name: String, current_score: i8) -> (ScoreboardState, i8) {
    let mut scores: Vec<String> = vec![];
    let mut lines: Vec<String> = vec![];
    let mut number_scores: Vec<i8> = vec![];
    let string_remove = format!("{name} has score: ");
    let scoreboard = decode::decode();
    let mut counter = 0;
    debug!("searching for lines with name");
    for l in scoreboard.lines() {
        if l.contains(&name) {
            lines.push(l.to_string());
        }
    }
    debug!("Getting scores");
    for s in lines.clone() {
        scores.push(s.replace(&string_remove, ""));
        counter += 1;
    }
    debug!("converting scoreboard to UTF8");
    for s in scores {
        match s.trim().parse::<i8>() {
            Ok(t) => number_scores.push(t),
            Err(e) => {
                error!("Scoreboard has invalid data: ({e})");
                panic!();
            }
        };
    }
    debug!("Checking if score is already in scoreboard");
    for n in number_scores.iter() {
        if *n == current_score {
            debug!("Found the number in scoreboard");
            return (ScoreboardState::NoAdding, 0);
        }
    }
    if counter < 3 {
        info!("Less than 3 scores of {name} found.");
        return (ScoreboardState::JustAdding, 0);
    }

    debug!("Getting smallest number");
    let smallest = number_scores.iter().min();
    if *smallest.unwrap() > current_score {
        (ScoreboardState::NoAdding, 0)
    } else {
        (ScoreboardState::ReplaceAdding, *smallest.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::check_for_scoreboard;
    use base64::prelude::*;
    use std::fs::*;
    use std::io::*;
    use std::io::ErrorKind;
    use std::path::Path;
    const SCOREBOARD_PATH: &str = "data/guessing_game/scores.dat";
    const PANIC_MESSAGE: &str = "Failed to create scoreboard: ";
    #[allow(dead_code)]
    fn setup() {
        println!("Initializing test environment");
        println!("Stored scores will be deleted");
        debug!("Check for scoreboard");
        loop {
            if !check_for_scoreboard::check_if_path_exist() {
                match File::create(SCOREBOARD_PATH) {
                    Ok(_) => {}
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => {
                            create_dir("data/").unwrap_or_else(|e| match e.kind() {
                                ErrorKind::AlreadyExists => create_dir("data/guessing_game/")
                                    .unwrap_or_else(|e| {
                                        error!("{PANIC_MESSAGE} {}", e);
                                        panic!()
                                    }),
                                _ => {
                                    error!("{PANIC_MESSAGE} {}", e);
                                    panic!()
                                }
                            })
                        }
                        _ => {
                            error!("{PANIC_MESSAGE} {}", e);
                            panic!()
                        }
                    },
                }
                if check_for_scoreboard::check_if_path_exist() {
                    break;
                }
            } else {
                add_value_to_scoreboard(String::new());
                break;
            }
        }
    }
    #[allow(dead_code)]
    fn add_value_to_scoreboard(addend: String) {
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
        match file.write_all(BASE64_STANDARD.encode(addend).as_bytes()) {
            Ok(f) => {
                info!("Added score");
                f
            }
            Err(e) => {
                error!("Error while adding score. Exiting because: {}", e);
                panic!()
            }
        }
        
    }
    #[test]
    fn test_remove_scores_just_adding() {
        setup();
        let name = "test".to_string();
        let current_score = 10;
        let (state, score) = remove_scores(name, current_score);
        assert_eq!(state, ScoreboardState::JustAdding);
        assert_eq!(0, score);
    }
    #[test]
    fn test_remove_scores_no_adding() {
        setup();
        let scoreboard = String::new();
        let score = 10;
        let name = "test".to_string();
        add_value_to_scoreboard(format!("{scoreboard} \n {name} has score: {score}\n"));
        let current_score = 10;
        let (state, score) = remove_scores(name, current_score);
        assert_eq!(state, ScoreboardState::NoAdding);
        assert_eq!(0, score);
    }
    #[test]
    fn test_remove_scores_replace_adding() {
        setup();
        let name = "test".to_string();
        let addend:String = format!("{name} has score: 1\n {name} has score: 2\n {name} has score: 3\n");
        add_value_to_scoreboard(addend);
        let current_score = 10;
        let (state, score) = remove_scores(name, current_score);
        assert_eq!(state, ScoreboardState::ReplaceAdding);
        assert_eq!(1, score);
    }
}

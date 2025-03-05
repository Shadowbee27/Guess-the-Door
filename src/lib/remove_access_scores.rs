use crate::lib::decode;
use log::{debug, error, info};
use std::cmp::*;

#[derive(PartialEq)]
pub enum ScoreboardState {
    ReplaceAdding,
    NoAdding,
    JustAdding,
}
pub fn remove_scores(name: String, current_score: i8) -> (ScoreboardState, i8) {
    let mut scores = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let scoreboard = decode::decode();
    let mut number_scores: Vec<i8> = Vec::new();
    let string_remove = format!("{name} has score: ");
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
    if counter < 3 {
        info!("Less than 3 scores of {name} found.");
        (ScoreboardState::JustAdding, 0)
    } else {
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
        debug!("Getting smallest number");
        let smallest = number_scores.iter().min();
        if *smallest.unwrap() > current_score {
            (ScoreboardState::NoAdding, 0)
        } else {
            (ScoreboardState::ReplaceAdding, *smallest.unwrap())
        }
    }
}

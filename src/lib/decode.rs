use crate::lib::check_for_scoreboard;
use base64::prelude::*;
use std::fs::*;

const SCOREBOARD_PATH: &str = "src/gamefiles/scoreboard.dat";
pub fn decode() -> String {
    if !check_for_scoreboard::check_if_path_exist() {
        println!("No score board found");
        return String::new();
    }
    let mut scoreboard: Vec<u8> = read(SCOREBOARD_PATH).unwrap();

    scoreboard = BASE64_STANDARD.decode(scoreboard).unwrap();
    let string_scoreboard: &str = match std::str::from_utf8(scoreboard.as_slice()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    string_scoreboard.trim().to_string()
}

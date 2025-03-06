use crate::lib::check_for_scoreboard;
use base64::prelude::*;
use log::{debug, error, info};
use std::fs::*;

const SCOREBOARD_PATH: &str = "src/gamefiles/scoreboard.dat";
pub fn decode() -> String {
    debug!("Checking for Path of scoreboard");
    if !check_for_scoreboard::check_if_path_exist() {
        info!("No score board found");
        return String::new();
    }
    let mut scoreboard: Vec<u8> = read(SCOREBOARD_PATH).unwrap();
    debug!("Decoding Scoreboard");
    scoreboard = BASE64_STANDARD.decode(scoreboard).unwrap();
    debug!("Converting Scoreboard to UTF8");
    let string_scoreboard: &str = match std::str::from_utf8(scoreboard.as_slice()) {
        Ok(v) => v,
        Err(e) => {
            error!("Invalid UTF-8 sequence: {}", e);
            panic!();
        }
    };
    string_scoreboard.trim().to_string()
}

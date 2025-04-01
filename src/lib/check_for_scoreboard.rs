use std::path::Path;
const SCOREBOARD_PATH: &str = "data/guessing_game/scores.dat";
pub fn check_if_path_exist() -> bool {
    Path::exists(SCOREBOARD_PATH.as_ref())
}

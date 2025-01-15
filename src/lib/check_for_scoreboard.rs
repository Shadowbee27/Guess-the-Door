use std::path::Path;
const SCOREBOARD_PATH: &str = "src/gamefiles/scoreboard.dat";
pub fn check_if_path_exist() -> bool {
    Path::exists(SCOREBOARD_PATH.as_ref())
}

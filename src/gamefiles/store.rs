use std::fs::*;
use std::path::Path;
const SCOREBOARD_PATH: &str = "src/gamefiles/scoreboard.txt";
pub fn store(name: String, score: i8) {
    if !check_if_path_exist(SCOREBOARD_PATH) {
        println!("Scoreboard file not found, creating it now.");
        match File::create(SCOREBOARD_PATH) {
            Ok(_) => {
                println!("created scoreboard successfully");
                ()
            }
            Err(e) => panic!("Error creating scoreboard. This is a state of no recovery, the program will crash. Exiting because: {}", e),
        }
    } else {
        println!("Found scoreboard");
    }
    let mut file = match File::open(SCOREBOARD_PATH) {
        Ok(file) => file,
        Err(e) => panic!("File error. {}", e),
    };
    let binding = score.to_string();
    let string_score=binding.as_str();
    let add_score:String = format!("{name} has score: {score}\n");
    println!("Adding score: {}", add_score);
}
fn check_if_path_exist(file_name: &str) -> bool {
    Path::exists(file_name.as_ref())
}
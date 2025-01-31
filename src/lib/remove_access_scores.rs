use crate::lib::decode;
#[derive(PartialEq)]
pub enum ScoreboardState {
    ReplaceAdding,
    NoAdding,
    JustAdding,
}
pub fn remove_scores(name: String) -> (ScoreboardState, String) {
    let mut scores = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let scoreboard = decode::decode();
    let mut number_scores: Vec<i8> = Vec::new();
    let string_remove = format!("{name} has score: ");
    let mut counter = 0;
    for l in scoreboard.lines() {
        if l.contains(&name) {
            lines.push(l.to_string());
        }
    }
    println!("Here{:?}", lines);
    for s in lines.clone() {
        scores.push(s.replace(&string_remove, ""));
        counter += 1;
    }
    if counter < 3 || counter == 0 {
        println!("Less than 3 scores of {name} found.");
        (ScoreboardState::JustAdding, "".to_string())
    } else {
        for s in scores {
            match s.trim().parse::<i8>() {
                Ok(t) => number_scores.push(t),
                Err(e) => panic!("Scoreboard has invalid data: ({e})"),
            };
        }
        println!("iSIUAGLUAHDRG {:?}", number_scores);
        let smallest = number_scores.iter().min();
        let option_score: Vec<i8> = Vec::new();
        if smallest == option_score.iter().min() || smallest < option_score.iter().min() {
            (ScoreboardState::NoAdding, "".to_string())
        } else {
            let test = format!(" {name} has score: {}\n", smallest.unwrap());
            println!("Z {test}");
            (
                ScoreboardState::ReplaceAdding,
                format!(" {name} has score: {}\n", smallest.unwrap()),
            )
        }
    }
}

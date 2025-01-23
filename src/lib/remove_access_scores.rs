use crate::lib::decode;
pub fn remove_scores(name: String, score: i8) {
    let mut scores = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let scoreboard = decode::decode();
    let string_remove = format!("{name} has score: ");
    for l in scoreboard.lines() {
        if l.contains(&name) == true {
            lines.push(l.to_string());
        }
        for s in lines.clone() {
            scores.push(s.replace(&string_remove, ""))
        }
    }
    println!("lines with name {:?}", scores);
}


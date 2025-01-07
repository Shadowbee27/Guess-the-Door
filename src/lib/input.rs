use std::io;

pub fn menu_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
pub fn int_input() -> i64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<i64>();
        match input {
            Ok(ok) => {
                if matches!(ok, 0..4) {
                    return ok;
                } else {
                    eprintln!("Number out of range.");
                }
            }
            Err(e) => eprintln!("Error: ({}). Please try again", e),
        }
    }
}

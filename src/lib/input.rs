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
                    println!("Invalid input.");
                }
            }
            Err(e) => println!("Error: ({}). Please try again", e),
        }
    }
}

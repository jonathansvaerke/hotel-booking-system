use std::io;

pub fn get_string(command: &str) -> String {
    let mut input = String::new();
    eprint!("{}", command);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn get_int(command: &str) -> u32 {
    let mut input = String::new();
    eprint!("{}", command);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
        .trim()
        .parse::<u32>()
        .expect("Failed integer conversion")
}

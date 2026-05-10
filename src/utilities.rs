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
    loop {
        let mut input = String::new();
        eprint!("{}", command);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_e) => println!("Could not convert int. Please try again"),
        }
    }
}

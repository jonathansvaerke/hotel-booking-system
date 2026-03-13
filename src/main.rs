use rusqlite::{Connection, Result};
use std::io;

fn main() -> Result<()> {
    init_database()?;

    println!("HBS Online");
    loop {
        let command = get_input("Command > ");
        match command.as_str() {
            "addroom" | "ar" => add_room(),
            "addguest" | "ag" => add_guest(),
            "exit" | "quit" | "q" => break,
            "help" | "h" => help(),
            _ => println!("Unknown command"),
        }
    }

    Ok(())
}

fn add_room() {
    let _number = get_input("  Room number > ");
    let _class = get_input("  Room class > ");
    let _capacity = get_input("  Room capacity > ");
    let _price: i32 = get_input("  Room price > ")
        .parse()
        .expect("Failed integer conversion");
    //     conn.execute(
    //         "INSERT INTO rooms (number, class, capacity, price)
    //         VALUES (?1, ?2, ?3, ?4)",
    //         "",
    //     )
    println!("  Room added");
}

fn add_guest() {
    let _first_name = get_input("  First name > ");
    let _last_name = get_input("  Last name > ");
    let _email = get_input("  Email > ");
    let _phone = get_input("  Phone number > ");
    //     conn.execute(
    //         "INSERT INTO rooms (number, class, capacity, price)
    //         VALUES (?1, ?2, ?3, ?4)",
    //         "",
    //     )
    println!("  Guest added")
}

fn help() {
    // Show documentation
    println!("<Documentation>");
    println!("Skill issue - get better");
}

fn init_database() -> Result<()> {
    let conn = Connection::open("hotel-database.db")?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    /*
        rooms(PK, number, class, capacity)
        guests(PK, name, email, phone)
        booking(PK, start_date, end_date)
    */
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS rooms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            number INTEGER NOT NULL UNIQUE,
            class INTEGER NOT NULL,
            capacity INTEGER NOT NULL,
            price REAL NOT NULL
        );

        CREATE TABLE IF NOT EXISTS guests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            phone Text NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS bookings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            room_id INTEGER NOT NULL,
            guest_id INTEGER NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL,
            CHECK (end_date > start_date),
            FOREIGN KEY (room_id) REFERENCES rooms(id),
            FOREIGN KEY (guest_id) REFERENCES guests(id)
        );
        ",
    )?;

    Ok(())
}

fn get_input(command: &str) -> String {
    let mut input = String::new();
    eprint!("{}", command);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

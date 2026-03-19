use rusqlite::{Connection, Result};

use crate::utilities::{get_int, get_string};

pub fn add_guest(conn: &Connection) -> Result<()> {
    let first_name = get_string("  First name > ");
    let last_name = get_string("  Last name > ");
    let email = get_string("  Email > ");
    let phone = get_string("  Phone number > ");
    conn.execute(
        "INSERT INTO guests (first_name, last_name, email, phone)
            VALUES (?1, ?2, ?3, ?4)",
        [&first_name, &last_name, &email, &phone],
    )?;
    println!("  Guest {} {} added", first_name, last_name);
    Ok(())
}

pub fn remove_guest(conn: &Connection) -> Result<()> {
    let remove_type = get_string("Type e for email or p for phone number > ");
    loop {
        match remove_type.as_str() {
            "e" => {
                let email = get_string("  Email > ");
                conn.execute("DELETE FROM guests WHERE email = ?1", [&email])?;
                println!("  Guest removed");
                break;
            }
            "p" => {
                let phone = get_string("  Phone number > ");
                conn.execute("DELETE FROM guests WHERE phone = ?1", [&phone])?;
                println!("  Guest removed");
                break;
            }
            "exit" => break,
            _ => println!("Unknown command. Please type e or t"),
        };
    }
    Ok(())
}

pub fn add_room(conn: &Connection) -> Result<()> {
    let number: u32 = get_int("  Room number > ");
    let class: u32 = get_int("  Room class > ");
    let capacity: u32 = get_int("  Room capacity > ");
    let price: u32 = get_int("  Room price > ");

    conn.execute(
        "INSERT INTO rooms (number, class, capacity, price)
            VALUES (?1, ?2, ?3, ?4)",
        (&number, &class, &capacity, &price),
    )?;
    println!("  Room added");
    Ok(())
}

pub fn remove_room(conn: &Connection) -> Result<()> {
    let number: u32 = get_int("  Room number > ");
    conn.execute("DELETE FROM rooms WHERE number = ?1", [&number])?;
    println!(" Room {} removed", number);
    Ok(())
}

pub fn show(conn: &Connection) -> Result<()> {
    loop {
        let table = get_string("Table name > ");

        match table.as_str() {
            "guests" => {
                conn.execute("SELECT * FROM guests", [])?;
                break;
            }
            "rooms" => {
                conn.execute("SELECT * FROM rooms", [])?;
                break;
            }
            "bookings" => {
                conn.execute("SELECT * FROM guests", [])?;
                break;
            }
            _ => println!("Unknown table name. Choose between guests, rooms or bookings"),
        }
    }
    Ok(())
}

pub fn help() {
    // Show documentation
    println!("Skill issue - get better. But just because you asked nicely ;)"); // Remove this
    println!("The following commands are available:");
    println!("  - addguest or ag | Use this command to add a guest to the database");
    println!("  - removeguest or rg | Use this command to remove a guest from the database");
    println!("  - addroom or ar | Use this command to add a room to the database");
    println!("  - removeroom or rr | Use this command to remove a room from the database");
    println!("  - exit or q | Use this command to exit the program")
}

use rusqlite::{Connection, Result};

use crate::{
    query::{query_bookings, query_guests, query_rooms},
    utilities::{get_int, get_string},
};

pub fn add_guest(conn: &Connection) -> Result<()> {
    let first_name = get_string("  First name > ");
    let last_name = get_string("  Last name > ");
    let email = get_string("  Email > ");
    let phone = get_string("  Phone number > ");
    match conn.execute(
        "INSERT INTO guests (first_name, last_name, email, phone)
            VALUES (?1, ?2, ?3, ?4)",
        [&first_name, &last_name, &email, &phone],
    ) {
        Ok(_) => println!("    Guest added successfully"),
        Err(e) => println!("    Error whille adding guest: {}", e),
    }
    Ok(())
}

pub fn remove_guest(conn: &Connection) -> Result<()> {
    let remove_type = get_string("Type e for email or p for phone number > ");
    loop {
        match remove_type.as_str() {
            "e" => {
                let email = get_string("  Email > ");
                match conn.execute("DELETE FROM guests WHERE email = ?1", [&email]) {
                    Ok(0) => println!("   No guest found with that email"),
                    Ok(_) => println!("   Guest removed"),
                    Err(e) => println!("   Error removing guest: {}", e),
                }
                break;
            }
            "p" => {
                let phone = get_string("  Phone number > ");
                match conn.execute("DELETE FROM guests WHERE email = ?1", [&phone]) {
                    Ok(0) => println!("   No guest found with that phone number"),
                    Ok(_) => println!("   Guest removed"),
                    Err(e) => println!("   Error removing guest: {}", e),
                }
                break;
            }
            "exit" => break,
            _ => println!("Unknown command. Please type e or p"),
        };
    }
    Ok(())
}

pub fn add_room(conn: &Connection) -> Result<()> {
    let number: u32 = get_int("  Room number > ");
    let class: u32 = get_int("  Room class > ");
    let capacity: u32 = get_int("  Room capacity > ");
    let price: u32 = get_int("  Room price > ");
    match conn.execute(
        "INSERT INTO rooms (number, class, capacity, price)
            VALUES (?1, ?2, ?3, ?4)",
        [&number, &class, &capacity, &price],
    ) {
        Ok(_) => println!("    Room added successfully"),
        Err(e) => println!("    Error removing room: {}", e),
    }
    Ok(())
}

pub fn remove_room(conn: &Connection) -> Result<()> {
    let number: u32 = get_int("  Room number > ");
    match conn.execute("DELETE FROM rooms WHERE number = ?1", [&number]) {
        Ok(0) => println!("    No room found with number with room number"),
        Ok(_) => println!("    Room removed"),
        Err(e) => println!("    Error removing room: {}", e),
    }
    Ok(())
}

pub fn add_booking(conn: &Connection) -> Result<()> {
    let room_number = get_int("    Room number > ");
    let room_id: i64 = conn.query_row(
        "SELECT id FROM rooms WHERE number = ?1",
        [room_number],
        |row| row.get(0),
    )?;

    let guest_phone = get_string("    Guest phone number > ");
    let guest_id: i64 = conn.query_row(
        "SELECT id FROM guests WHERE phone = ?1",
        [guest_phone],
        |row| row.get(0),
    )?;

    println!("Date format: <YEAR-MM-DD>");
    let start_date = get_string("    Start date > ");
    let end_date = get_string("    End date > ");

    match conn.execute(
        "INSERT INTO bookings (room_id, guest_id, start_date, end_date) VALUES (?1, ?2, ?3, ?4)",
        (&room_id, &guest_id, &start_date, &end_date), // Tuples are being used here because, the data have different types.
    ) {
        Ok(_) => println!("Booking created successfully"),
        Err(e) => println!("Error creating booking: {}", e),
    }

    Ok(())
}

pub fn remove_booking(_conn: &Connection) -> Result<()> {
    Ok(())
}

pub fn show(conn: &Connection) -> Result<()> {
    loop {
        let table = get_string("Table name > ");
        match table.as_str() {
            "guests" => query_guests(conn)?,
            "rooms" => query_rooms(conn)?,
            "bookings" => query_bookings(conn)?,
            "exit" => break,
            _ => {
                println!("Unknown table. Choose between guests, rooms, bookings or exit");
                continue;
            }
        };
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

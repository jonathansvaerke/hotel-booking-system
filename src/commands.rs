use rusqlite::{Connection, Result};

use crate::{
    query::{query_bookings, query_guests, query_rooms},
    structs::Booking,
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
        Err(e) => println!("    Error while adding guest: {}", e),
    }
    Ok(())
}

pub fn remove_guest(conn: &Connection) -> Result<()> {
    loop {
        let remove_type = get_string("Type e for email or p for phone number > ");
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
                match conn.execute("DELETE FROM guests WHERE phone = ?1", [&phone]) {
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
    )?; // In this part of the function, one could also ask whether to use email or phone number. Like in removeguest.

    let guest_phone = get_string("    Guest phone number > ");
    let guest_id: i64 = conn.query_row(
        "SELECT id FROM guests WHERE phone = ?1",
        [guest_phone],
        |row| row.get(0),
    )?; // This could be made better by using a match-statement. Same with the above. Remember to add case Ok(0) for 'user not found error'

    let (start_date, end_date) = loop {
        println!("Date format: <YEAR-MM-DD>");
        let start = get_string("    Start date > ");
        let end = get_string("    End date > ");
        if start < end {
            break (start, end);
        }
        println!("    Start date must be before end date");
    };

    let mut stmt = conn.prepare("SELECT start_date, end_date FROM bookings WHERE room_id = ?1")?;
    let booking_iter = stmt.query_map([room_id], |row| {
        Ok(Booking {
            room_id: 0,
            guest_id: 0,
            start_date: row.get(0)?,
            end_date: row.get(1)?,
        })
    })?;

    for booking in booking_iter {
        let booking = booking?;
        // This check if the booking is valid
        if end_date.as_str() <= booking.start_date.as_str()
            || start_date.as_str() >= booking.end_date.as_str()
        {
            continue; // The booking is alright
        } else {
            println!("This booking intervenes with an existing booking");
            return Ok(());
        }
    }

    match conn.execute(
        "INSERT INTO bookings (room_id, guest_id, start_date, end_date) VALUES (?1, ?2, ?3, ?4)",
        (&room_id, &guest_id, &start_date, &end_date), // Tuples are being used here because, the data slots have different types.
    ) {
        Ok(_) => println!("Booking created successfully"),
        Err(e) => println!("Error creating booking: {}", e),
    }

    Ok(())
}

pub fn remove_booking(_conn: &Connection) -> Result<()> {
    /* Hent alle datoer på EN gæsts bookinger på ET rum. Sammen med datoerne, print booking id.
    Derefter kan brugeren indtaste den korrekte booking id. Og databasen sletter booking med det id. */
    println!("This function is not yet operational");
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
    // Show documentation. This could also be refactored to look more nicely, and retain performance by using a BuffWriter.
    println!(
        "Skill issue - get better. But just because you asked nicely ;)\n
The following commands are available:\n
    - addguest or ag | Use this command to add a guest to the database\n
    - removeguest or rg | Use this command to remove a guest from the database\n
    - addroom or ar | Use this command to add a room to the database\n
    - removeroom or rr | Use this command to remove a room from the database\n
    - exit or q | Use this command to exit the program"
    );
}

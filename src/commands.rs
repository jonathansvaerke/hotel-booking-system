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
        (&first_name, &last_name, &email, &phone),
    )?;
    println!("  Guest {} {} added", first_name, last_name);
    Ok(())
}

pub fn remove_guest(conn: &Connection) -> Result<()> {
    // Ask whether to identify guest by phone number or email, then delete if guest exists
    //println!("  Guest {} {} removed", first_name, last_name);
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
    println!(" Room {} removed", number);
    Ok(())
}

pub fn help() {
    // Show documentation
    println!("<Documentation>");
    println!("Skill issue - get better");
}

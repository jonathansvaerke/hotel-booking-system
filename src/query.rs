use rusqlite::{Connection, Result};

use crate::structs::{Guest, Room};
//use crate::utilities::{get_int, get_string};

pub fn query_guests(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, first_name, last_name, email, phone FROM guests")?;
    let guest_iter = stmt.query_map([], |row| {
        Ok(Guest {
            //id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            email: row.get(3)?,
            phone: row.get(4)?,
        })
    })?;

    for guest in guest_iter {
        let guest = guest?;
        println!(
            "    Name: {} {} | Email: {} | Phone: {}",
            guest.first_name, guest.last_name, guest.email, guest.phone
        );
    }
    Ok(())
}

pub fn query_rooms(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, number, class, capacity, price FROM rooms")?;
    let room_iter = stmt.query_map([], |row| {
        Ok(Room {
            //id: row.get(0)?,
            number: row.get(1)?,
            class: row.get(2)?,
            capacity: row.get(3)?,
            price: row.get(4)?,
        })
    })?;
    for room in room_iter {
        let room = room?;
        println!(
            "    Number: {} | Class: {} | Capacity: {} | Price: {}",
            room.number, room.class, room.capacity, room.price
        );
    }
    Ok(())
}

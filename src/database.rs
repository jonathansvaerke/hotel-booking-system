use rusqlite::{Connection, Result};

pub fn init_database() -> Result<Connection> {
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
    Ok(conn)
}

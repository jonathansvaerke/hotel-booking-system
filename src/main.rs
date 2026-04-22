use rusqlite::Result;

mod commands;
mod database;
mod query;
mod structs;
mod utilities;

/*
    To see all entries in table > SELECT * FROM table_name;
*/

fn main() -> Result<()> {
    let conn = database::init_database()?;

    println!("HBS Online");
    loop {
        let command = utilities::get_string("Command > ");
        match command.as_str() {
            "addguest" | "ag" => commands::add_guest(&conn)?,
            "removeguest" | "rg" => commands::remove_guest(&conn)?,
            "addroom" | "ar" => commands::add_room(&conn)?,
            "removeroom" | "rr" => commands::remove_room(&conn)?,
            "addbooking" | "ab" => commands::add_booking(&conn)?,
            "removebooking" | "rb" => commands::remove_booking(&conn)?,
            "show" | "s" => commands::show(&conn)?,
            "exit" | "quit" | "q" => break,
            "help" | "h" => commands::help(),
            _ => println!("Unknown command"),
        }
    }
    Ok(())
}

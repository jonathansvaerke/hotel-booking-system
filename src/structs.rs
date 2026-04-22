#[derive(Debug)]

pub struct Guest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}

pub struct Room {
    pub number: u32,
    pub class: u32,
    pub capacity: u32,
    pub price: f64,
}

pub struct Booking {
    pub room_id: u32,
    pub guest_id: u32,
    pub start_date: String,
    pub end_date: String,
}

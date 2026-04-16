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

use kingler::Kingler;
use serde::Serialize;

#[derive(Serialize)]
pub struct Client {
    id: u32,
    name: String,
    age: u8,
}

#[derive(Serialize)]
pub struct Product {
    id: u32,
    name: String,
    price: u8,
}

fn main() {
    let kingler = Kingler::new("sqlite".to_string(), "database.db".to_string());

    kingler.create_table(Client{
        id: 0,
        name: "".to_string(),
        age: 0,
    });
    kingler.create_table(Product{
        id: 0,
        name: "".to_string(),
        price: 0,
    });
}

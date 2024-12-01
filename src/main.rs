use kingler::Kingler;
use serde::Serialize;

#[derive(Serialize)]
pub struct Client {
    name: String,
    age: u8,
}

#[derive(Serialize)]
pub struct Product {
    name: String,
    price: u8,
}

fn main() {
    let kingler = Kingler::new("sqlite".to_string(), "database.db".to_string());

    kingler.create_table(Client{
        name: "".to_string(),
        age: 0,
    });
    kingler.create_table(Product{
        name: "".to_string(),
        price: 0,
    });
}

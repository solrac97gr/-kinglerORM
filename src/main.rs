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

    kingler.insert(&Client{
        id: 1,
        name: "John Doe".to_string(),
        age: 25,
    });

    kingler.insert(&Product{
        id: 1,
        name: "Apple".to_string(),
        price: 10,
    });
}

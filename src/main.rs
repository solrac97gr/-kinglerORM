use kingler::Kingler;
use serde::Serialize;

#[derive(Serialize)]
pub struct Client {
    id: Option<u32>,
    name: String,
    age: u8,
}

#[derive(Serialize)]
pub struct Product {
    id: Option<u32>,
    name: String,
    price: u8,
}

fn main() {
    let kingler = Kingler::new("sqlite".to_string(), "database.db".to_string());

    println!("Creating Client table...");
    kingler.create_table(Client{
        id: None,
        name: "".to_string(),
        age: 0,
    }).unwrap();

    println!("Creating Product table...");
    kingler.create_table(Product{
        id: None,
        name: "".to_string(),
        price: 0,
    }).unwrap();

    let client_id = kingler.insert(&Client{
        id: None,
        name: "John Doe".to_string(),
        age: 25,
    }).unwrap();

    let product_id = kingler.insert(&Product{
        id: None,
        name: "Apple".to_string(),
        price: 10,
    }).unwrap();

    let product_id2 = kingler.insert(&Product{
        id: None,
        name: "Banana".to_string(),
        price: 9,
    }).unwrap();

    let client_id2 = kingler.insert(&Client{
        id: None,
        name: "Jane Doe".to_string(),
        age: 25,
    }).unwrap();

    println!("Inserted client with ID: {}", client_id);
    println!("Inserted product with ID: {}", product_id);
    println!("Inserted product with ID: {}", product_id2);
    println!("Inserted client with ID: {}", client_id2);
}

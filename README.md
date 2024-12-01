# Kingler 🦀

Work in progress. 🚧

A lightweight, type-safe ORM (Object-Relational Mapping) library for Rust that simplifies database operations with a clean and intuitive API.

## Features

- 🦀 Type-safe database operations using Rust structs
- 🔄 Automatic table creation and schema generation
- 🎯 Simple and intuitive API
- 🔌 Support for multiple database backends (currently SQLite, MySQL coming soon)
- 🛡️ Built with Rust's safety guarantees

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kingler = "0.1.0"
```

## Quick Start

```rust
use kingler::Kingler;
use serde::Serialize;

#[derive(Serialize)]
struct Product {
    id: u32,
    name: String,
    price: u8,
}

fn main() {
    let kingler = Kingler::new("sqlite".to_string(), "database.db".to_string());
    // Create the table
    kingler.create_table(Product{
        id: 0,
        name: "".to_string(),
        price: 0,
    });

    // Insert a new product
    kingler.insert(&Product{
        id: 1,
        name: "Apple".to_string(),
        price: 10,
    });
}
```
## How it works

KinglerORM uses Rust's powerful type system to automatically generate tables and columns based on your structs. It also supports SQLite, MySQL, and more databases in the future.

### Supported types
The ORM automatically maps Rust types to SQL types:
- `String` → TEXT
- `u8`, `i32`, `u32`, etc. → INTEGER
- `f32`, `f64` → REAL
- `bool` → BOOLEAN

### Database support

- 🚧 [SQLite](https://www.sqlite.org/) (work in progress)
- 🚧 [MySQL](https://www.mysql.com/) (coming soon)
- 🚧 [PostgreSQL](https://www.postgresql.org/) (coming soon)
- 🚧 [MongoDB](https://www.mongodb.com/) (coming soon)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
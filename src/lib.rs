use serde::Serialize;
pub mod sqlite;

pub trait Table {
    fn table_name() -> &'static str;
    fn to_columns(&self) -> Vec<String>;
}
#[derive(Serialize)]


pub struct Kingler {
    database: String,
    uri: String,
}

impl Kingler {
    /// Creates a new instance of the Kingler ORM
    /// 
    /// # Arguments
    /// * `database` - The type of database ("sqlite" or "mysql")
    /// * `uri` - The connection string or file path
    /// 
    /// # Example
    /// ```rust
    /// let db = Kingler::new("sqlite".to_string(), "my_database.db".to_string());
    /// ```
    pub fn new(database: String, uri: String) -> Self {
        Kingler {
            database,
            uri,
        }
    }

    /// Internal helper function that converts a Rust struct into database column definitions
    /// 
    /// # Type Parameters
    /// * `T` - Any type that implements the Serialize trait
    /// 
    /// # Arguments
    /// * `value` - The struct instance to analyze
    /// 
    /// # Returns
    /// A vector of tuples containing column names and their SQL types
    fn generate_columns<T: Serialize>(value: T) -> Vec<(String, String)> {
        let mut columns = Vec::new();
        
        if let Ok(json_value) = serde_json::to_value(&value) {
            if let serde_json::Value::Object(map) = json_value {
                for (field_name, field_value) in map {
                    let sql_type = match field_value {
                        serde_json::Value::String(_) => "TEXT",
                        serde_json::Value::Number(n) => {
                            if n.is_i64() { "INTEGER" }
                            else { "REAL" }
                        },
                        serde_json::Value::Bool(_) => "BOOLEAN",
                        _ => "TEXT", // Default to TEXT for other types
                    };
                    columns.push((field_name, sql_type.to_string()));
                }
            }
        }
        columns
    }

    /// Creates a new database table based on a Rust struct
    /// 
    /// # Type Parameters
    /// * `T` - Any type that implements the Serialize trait
    /// 
    /// # Arguments
    /// * `value` - An instance of the struct to use as a template
    /// 
    /// # Example
    /// ```rust
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: i32,
    /// }
    /// 
    /// let db = Kingler::new("sqlite".to_string(), "my_database.db".to_string());
    /// db.create_table(User {
    ///     name: String::new(),
    ///     age: 0,
    /// });
    /// ```
    pub fn create_table<T: Serialize>(&self, value: T) {
        let type_name = std::any::type_name::<T>();
        let table_name = type_name.split("::").last().unwrap_or(type_name);
        println!("Creating table for {}", table_name);
        
        let columns = Self::generate_columns(value);

        // Handle different database types
        match self.database.as_str() {
            "sqlite" => {
                if let Ok(sqlite) = sqlite::Sqlite::new(self.uri.to_string()) {
                    let result = sqlite.create_table(
                        table_name.to_string(),
                        columns.into_iter()
                              .map(|(name, type_)| format!("{} {}", name, type_))
                              .collect()
                    );
                    println!("Table creation result: {:?}", result);
                }
            }
            "mysql" => {
                println!("MySQL database not supported yet");
            }
            _ => {
                eprintln!("Database {} not supported", self.database);
            }
        }
    }

    /// Inserts a record into the database table
    /// 
    /// # Type Parameters
    /// * `T` - Any type that implements the Serialize trait
    /// 
    /// # Arguments
    /// * `record` - The struct instance to insert
    /// 
    /// # Example
    /// ```rust
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: i32,
    /// }
    /// 
    /// let db = Kingler::new("sqlite".to_string(), "my_database.db".to_string());
    /// db.insert(&User {
    ///     name: "John".to_string(),
    ///     age: 30,
    /// });
    /// ```
    pub fn insert<T: Serialize>(&self, record: &T) {
        let type_name = std::any::type_name::<T>();
        let table_name = type_name.split("::").last().unwrap_or(type_name);
        
        match self.database.as_str() {
            "sqlite" => {
                if let Ok(json_value) = serde_json::to_value(&record) {
                    if let serde_json::Value::Object(map) = json_value {
                        let columns: Vec<String> = map.keys().cloned().collect();
                        let values: Vec<String> = map.values()
                            .map(|v| v.to_string().trim_matches('"').to_string())
                            .collect();
                        
                        if let Ok(sqlite) = sqlite::Sqlite::new(self.uri.to_string()) {
                            let _ = sqlite.insert(table_name.to_string(), columns, values);
                        }
                    }
                }
            }
            "mysql" => {
                println!("MySQL database not supported yet");
            }
            _ => {
                eprintln!("Database {} not supported", self.database);
            }
        }
    }
}
use serde::Serialize;
use rusqlite;
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
                // Handle ID field first
                if map.contains_key("id") {
                    columns.push(("id".to_string(), "INTEGER PRIMARY KEY AUTOINCREMENT".to_string()));
                }
                
                // Handle other fields
                for (field_name, field_value) in map {
                    if field_name != "id" {  // Skip id as it's already handled
                        let sql_type = match field_value {
                            serde_json::Value::String(_) => "TEXT",
                            serde_json::Value::Number(_) => "INTEGER",
                            serde_json::Value::Bool(_) => "BOOLEAN",
                            _ => "TEXT",
                        };
                        columns.push((field_name, sql_type.to_string()));
                    }
                }
            }
        }
        columns
    }

    fn format_columns(columns: Vec<(String, String)>) -> Vec<String> {
        columns.into_iter()
            .map(|(name, type_)| format!("{} {}", name, type_))
            .collect()
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
    pub fn create_table<T: Serialize>(&self, value: T) -> Result<(), rusqlite::Error> {
        let table_name = std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("unknown");
        
        println!("Creating table for {}", table_name);
        
        let columns = Self::generate_columns(value);
        let formatted_columns = Self::format_columns(columns);
        
        match self.database.as_str() {
            "sqlite" => {
                if let Ok(sqlite) = sqlite::Sqlite::new(self.uri.to_string()) {
                    return sqlite.create_table(table_name.to_string(), formatted_columns);
                }
                Ok(())
            }
            "mysql" => {
                println!("MySQL database not supported yet");
                Err(rusqlite::Error::ExecuteReturnedResults)
            }
            _ => {
                eprintln!("Database {} not supported", self.database);
                Err(rusqlite::Error::ExecuteReturnedResults)
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
    pub fn insert<T: Serialize>(&self, record: &T) -> Result<i64, rusqlite::Error> {
        let type_name = std::any::type_name::<T>();
        let table_name = type_name.split("::").last().unwrap_or(type_name);
        
        match self.database.as_str() {
            "sqlite" => {
                if let Ok(json_value) = serde_json::to_value(&record) {
                    if let serde_json::Value::Object(map) = json_value {
                        let mut columns: Vec<String> = Vec::new();
                        let mut values: Vec<String> = Vec::new();
                        
                        // Skip id field if it's None
                        for (key, value) in map.iter() {
                            if key == "id" {
                                if let serde_json::Value::Null = value {
                                    continue;
                                }
                            }
                            columns.push(key.clone());
                            match value {
                                serde_json::Value::Number(n) => {
                                    if n.is_i64() {
                                        values.push(n.as_i64().unwrap().to_string())
                                    } else if n.is_u64() {
                                        values.push(n.as_u64().unwrap().to_string())
                                    } else {
                                        values.push(n.as_f64().unwrap().to_string())
                                    }
                                },
                                serde_json::Value::String(s) => values.push(format!("'{}'", s)),
                                serde_json::Value::Bool(b) => values.push(b.to_string()),
                                serde_json::Value::Null => values.push("NULL".to_string()),
                                _ => values.push(value.to_string()),
                            }
                        }
                        
                        if let Ok(sqlite) = sqlite::Sqlite::new(self.uri.to_string()) {
                            return sqlite.insert(table_name.to_string(), columns, values);
                        }
                    }
                }
                Err(rusqlite::Error::ExecuteReturnedResults)
            }
            "mysql" => {
                Err(rusqlite::Error::ExecuteReturnedResults)
            }
            _ => {
                Err(rusqlite::Error::ExecuteReturnedResults)
            }
        }
    }
}
/// Represents a connection to a SQLite database
/// 
/// This struct wraps the rusqlite Connection type and provides
/// high-level operations for table management and data manipulation.
pub struct Sqlite {
    /// The underlying SQLite connection
    conn: rusqlite::Connection,
}

impl Sqlite {
    /// Creates a new SQLite connection
    /// 
    /// # Arguments
    /// * `database_path` - Path to the SQLite database file. If the file doesn't exist,
    ///                     it will be created automatically.
    /// 
    /// # Returns
    /// * `Result<Self, rusqlite::Error>` - A Result containing either the Sqlite instance
    ///                                     or a database error
    /// 
    /// # Example
    /// ```rust
    /// let db = Sqlite::new("my_database.db".to_string())?;
    /// ```
    pub fn new(database_path: String) -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open(&database_path)?;
        Ok(Sqlite {
            conn,
        })
    }

    /// Creates a new table in the database if it doesn't already exist
    /// 
    /// # Arguments
    /// * `table_name` - Name of the table to create
    /// * `columns` - Vector of column definitions (e.g., "name TEXT", "age INTEGER")
    ///               Note: An 'id' column with AUTO INCREMENT is automatically added
    /// 
    /// # Returns
    /// * `Result<(), rusqlite::Error>` - Success (()) or a database error
    /// 
    /// # Example
    /// ```rust
    /// let db = Sqlite::new("my_database.db")?;
    /// db.create_table(
    ///     "users".to_string(),
    ///     vec![
    ///         "name TEXT".to_string(),
    ///         "age INTEGER".to_string()
    ///     ]
    /// )?;
    /// ```
    pub fn create_table(&self, table_name: String, columns: Vec<String>) -> Result<(), rusqlite::Error> {
        // Add an auto-incrementing primary key 'id' column to the start of the columns list
        let columns_with_id = std::iter::once("id INTEGER PRIMARY KEY AUTOINCREMENT".to_string())
            .chain(columns)
            .collect::<Vec<String>>();
        // Join all column definitions with commas
        let columns_str = columns_with_id.join(", ");
        
        // Construct the CREATE TABLE SQL query
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns_str);
        
        // Execute the query ([] means no parameters are needed)
        self.conn.execute(&query, [])?;
        
        Ok(())
    }

    /// Inserts a new record into a specified table
    /// 
    /// # Arguments
    /// * `table_name` - Name of the target table
    /// * `columns` - Vector of column names to insert into
    /// * `values` - Vector of values to insert (must match columns in length)
    /// 
    /// # Returns
    /// * `Result<(), rusqlite::Error>` - Success (()) or a database error
    /// 
    /// # Example
    /// ```rust
    /// let db = Sqlite::new("my_database.db")?;
    /// db.insert(
    ///     "users".to_string(),
    ///     vec!["name".to_string(), "age".to_string()],
    ///     vec!["John".to_string(), "30".to_string()]
    /// )?;
    /// ```
    pub fn insert(&self, table_name: String, columns: Vec<String>, values: Vec<String>) -> Result<(), rusqlite::Error> {
        // Create a string of SQL placeholders ("?, ?, ?") matching the number of values
        let placeholders = vec!["?"; columns.len()].join(", ");
        // Join column names with commas
        let columns_str = columns.join(", ");
        // Construct the INSERT SQL query
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, columns_str, placeholders
        );

        // Execute the query with the provided values
        self.conn.execute(&query, rusqlite::params_from_iter(values))?;
        Ok(())
    }
}


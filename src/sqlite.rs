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
        let columns_str = columns.join(", ");
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns_str);
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
    /// * `Result<i64, rusqlite::Error>` - Success (()) or a database error
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
    pub fn insert(&self, table_name: String, columns: Vec<String>, values: Vec<String>) -> Result<i64, rusqlite::Error> {
        let placeholders = vec!["?"; columns.len()].join(", ");
        let columns_str = columns.join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, columns_str, placeholders
        );

        // Convert string values to params
        let params: Vec<&dyn rusqlite::ToSql> = values.iter()
            .map(|v| v as &dyn rusqlite::ToSql)
            .collect();

        self.conn.execute(&query, rusqlite::params_from_iter(params))?;
        Ok(self.conn.last_insert_rowid())
    }
    /// Creates a database relationship between two tables
    /// 
    /// # Arguments
    /// * `table_name1` - Name of the first table in the relationship
    /// * `table_name2` - Name of the second table in the relationship
    /// * `column1` - Primary key column name in the first table
    /// * `column2` - Primary key column name in the second table
    /// * `relation_type` - Type of relationship to create. Must be one of:
    ///     - "MANY_TO_MANY": Creates a junction table named "{table1}_{table2}"
    ///     - "ONE_TO_MANY": Adds a foreign key to table2 (the "many" side)
    ///     - "ONE_TO_ONE": Adds a unique foreign key to table1
    /// 
    /// # Returns
    /// * `Result<(), rusqlite::Error>` - Success (()) or a database error
    /// 
    /// # Examples
    /// ```rust
    /// // Create a many-to-many relationship between users and roles
    /// db.create_relationship(
    ///     "users".to_string(),
    ///     "roles".to_string(),
    ///     "id".to_string(),
    ///     "id".to_string(),
    ///     "MANY_TO_MANY".to_string()
    /// )?;
    /// 
    /// // Create a one-to-many relationship between departments and employees
    /// db.create_relationship(
    ///     "departments".to_string(),
    ///     "employees".to_string(),
    ///     "id".to_string(),
    ///     "id".to_string(),
    ///     "ONE_TO_MANY".to_string()
    /// )?;
    /// 
    /// // Create a one-to-one relationship between users and profiles
    /// db.create_relationship(
    ///     "users".to_string(),
    ///     "profiles".to_string(),
    ///     "id".to_string(),
    ///     "id".to_string(),
    ///     "ONE_TO_ONE".to_string()
    /// )?;
    /// ```
    /// 
    /// # Details
    /// ## Many-to-Many
    /// Creates a junction table that contains foreign keys to both tables,
    /// allowing multiple records from each table to be associated with each other.
    /// 
    /// ## One-to-Many
    /// Modifies the second table (table_name2) to include a foreign key reference
    /// to the first table (table_name1). This allows multiple records in table2
    /// to be associated with a single record in table1.
    /// 
    /// ## One-to-One
    /// Adds a unique foreign key to the first table (table_name1) referencing
    /// the second table (table_name2). The UNIQUE constraint ensures that each
    /// record in table2 can only be associated with one record in table1.
    /// 
    /// # Errors
    /// Returns a `rusqlite::Error` if:
    /// * The tables don't exist
    /// * The columns don't exist
    /// * The relation_type is not one of the supported types
    /// * There are existing data conflicts
    /// * The database operation fails
    pub fn create_relationship(
        &self,
        table_name1: String,
        table_name2: String,
        column1: String,
        column2: String,
        relation_type: String
    ) -> Result<(), rusqlite::Error> {
        match relation_type.to_uppercase().as_str() {
            "MANY_TO_MANY" => {
                // Create a junction table for many-to-many relationship
                let junction_table = format!("{}_{}", table_name1.to_lowercase(), table_name2.to_lowercase());
                let query = format!(
                    "CREATE TABLE IF NOT EXISTS {} ({}_ref INTEGER REFERENCES {}({}), {}_ref INTEGER REFERENCES {}({}))",
                    junction_table,
                    table_name1.to_lowercase(),
                    table_name1,
                    column1,
                    table_name2.to_lowercase(),
                    table_name2,
                    column2
                );
                self.conn.execute(&query, [])?;
            },
            "ONE_TO_MANY" => {
                // Add foreign key to the "many" side
                let query = format!(
                    "ALTER TABLE {} ADD COLUMN {}_ref INTEGER REFERENCES {}({})",
                    table_name2, // The "many" side gets the foreign key
                    table_name1.to_lowercase(),
                    table_name1,
                    column1
                );
                self.conn.execute(&query, [])?;
            },
            "ONE_TO_ONE" => {
                // Add foreign key with UNIQUE constraint
                let query = format!(
                    "ALTER TABLE {} ADD COLUMN {}_ref INTEGER UNIQUE REFERENCES {}({})",
                    table_name1,
                    table_name2.to_lowercase(),
                    table_name2,
                    column2
                );
                self.conn.execute(&query, [])?;
            },
            _ => return Err(rusqlite::Error::ExecuteReturnedResults),
        }

        Ok(())
    }
}


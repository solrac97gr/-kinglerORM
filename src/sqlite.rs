pub struct Sqlite {
    database_path: String,
}

impl Sqlite {
    pub fn new(database_path: String) -> Self {
        Sqlite {
            database_path,
        }
    }
    // create a new table in the database
    pub fn create_table(&self, table_name: String, columns: Vec<String>) -> Result<(), rusqlite::Error> {
        let conn = rusqlite::Connection::open(&self.database_path)?;
        
        // Build the CREATE TABLE query
        let columns_str = columns.join(", ");
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns_str);
        
        // Execute the query
        conn.execute(&query, [])?;
        
        Ok(())
    }

}


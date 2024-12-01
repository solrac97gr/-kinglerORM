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
        
        // Add id column as primary key
        let id_column = "id INTEGER PRIMARY KEY AUTOINCREMENT";
        let columns_with_id = std::iter::once(id_column.to_string())
            .chain(columns)
            .collect::<Vec<String>>();
        let columns_str = columns_with_id.join(", ");
        
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns_str);
        
        conn.execute(&query, [])?;
        
        Ok(())
    }

}


pub struct Sqlite {
    conn: rusqlite::Connection,
}

impl Sqlite {
    pub fn new(database_path: String) -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open(&database_path)?;
        Ok(Sqlite {
            conn,
        })
    }
    // create a new table in the database
    pub fn create_table(&self, table_name: String, columns: Vec<String>) -> Result<(), rusqlite::Error> {
        let columns_with_id = std::iter::once("id INTEGER PRIMARY KEY AUTOINCREMENT".to_string())
            .chain(columns)
            .collect::<Vec<String>>();
        let columns_str = columns_with_id.join(", ");
        
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns_str);
        
        self.conn.execute(&query, [])?;
        
        Ok(())
    }

    pub fn insert(&self, table_name: String, columns: Vec<String>, values: Vec<String>) -> Result<(), rusqlite::Error> {
        let placeholders = vec!["?"; columns.len()].join(", ");
        let columns_str = columns.join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, columns_str, placeholders
        );

        self.conn.execute(&query, rusqlite::params_from_iter(values))?;
        Ok(())
    }

}


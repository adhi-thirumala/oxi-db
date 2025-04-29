use crate::error::{DbError, Result};
use crate::table::Table;
use crate::types::{Column, Key, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Database structure that manages tables and provides persistence
///
/// The `Database` struct is the main entry point for interacting with Oxi-DB.
/// It manages a collection of tables and handles serialization/deserialization
/// to and from disk.
///
/// # Examples
///
/// Creating a new database:
///
/// ```
/// use oxi_db::Database;
///
/// let db = Database::new("my_database.db");
/// ```
///
/// Opening an existing database:
///
/// ```
/// use oxi_db::Database;
///
/// let db = Database::open("my_database.db").expect("Failed to open database");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    /// Path where the database file is stored
    path: PathBuf,
    /// Collection of tables in the database
    tables: BTreeMap<String, Table>,
}

impl Database {
    /// Create a new database at the specified path
    ///
    /// This creates a new, empty database with no tables. The database is not
    /// written to disk until `save()` is called.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the database file will be stored
    ///
    /// # Examples
    ///
    /// ```
    /// use oxi_db::Database;
    ///
    /// let db = Database::new("my_database.db");
    /// ```
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            tables: BTreeMap::new(),
        }
    }

    /// Open an existing database from the specified path
    ///
    /// This loads a database from disk. The file must exist and be a valid
    /// Oxi-DB database file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the database file
    ///
    /// # Returns
    ///
    /// A `Result` containing the loaded database or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file does not exist
    /// - The file cannot be read
    /// - The file is not a valid Oxi-DB database
    ///
    /// # Examples
    ///
    /// ```
    /// use oxi_db::Database;
    ///
    /// let db = Database::open("my_database.db").expect("Failed to open database");
    /// ```
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(DbError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Database file not found",
            )));
        }

        let data = fs::read(path)?;
        let db: Database = bincode::deserialize(&data)?;

        Ok(db)
    }

    /// Save the database to disk
    ///
    /// This serializes the entire database and writes it to the path specified
    /// when the database was created or opened. If the parent directory does not
    /// exist, it will be created.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database cannot be serialized
    /// - The file cannot be written
    /// - The parent directory cannot be created
    ///
    /// # Examples
    ///
    /// ```
    /// use oxi_db::Database;
    ///
    /// let mut db = Database::new("my_database.db");
    /// db.save().expect("Failed to save database");
    /// ```
    pub fn save(&self) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let serialized = bincode::serialize(self)?;
        fs::write(&self.path, serialized)?;

        Ok(())
    }

    /// Create a new table in the database
    pub fn create_table(
        &mut self,
        name: impl Into<String>,
        columns: Vec<Column>,
        primary_key: Option<String>,
    ) -> Result<()> {
        let name = name.into();

        if self.tables.contains_key(&name) {
            return Err(DbError::TableExists);
        }

        let table = Table::new(name.clone(), columns, primary_key);
        self.tables.insert(name, table);

        self.save()
    }

    /// Drop a table from the database
    pub fn drop_table(&mut self, name: &str) -> Result<()> {
        if !self.tables.contains_key(name) {
            return Err(DbError::TableNotFound);
        }

        self.tables.remove(name);
        self.save()
    }

    /// Get a reference to a table
    pub fn get_table(&self, name: &str) -> Result<&Table> {
        self.tables
            .get(name)
            .ok_or(DbError::TableNotFound)
    }

    /// Get a mutable reference to a table
    pub fn get_table_mut(&mut self, name: &str) -> Result<&mut Table> {
        self.tables
            .get_mut(name)
            .ok_or(DbError::TableNotFound)
    }

    /// List all tables in the database
    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }

    /// Insert a row into a table
    pub fn insert(&mut self, table_name: &str, key: impl Into<Key>, values: Vec<Value>) -> Result<()> {
        let table = self.get_table_mut(table_name)?;
        table.insert(key, values)?;
        self.save()
    }

    /// Get a row from a table
    pub fn get(&self, table_name: &str, key: &Key) -> Result<&Row> {
        let table = self.get_table(table_name)?;
        table.get(key)
    }

    /// Update a row in a table
    pub fn update(&mut self, table_name: &str, key: &Key, values: Vec<Value>) -> Result<()> {
        let table = self.get_table_mut(table_name)?;
        table.update(key, values)?;
        self.save()
    }

    /// Delete a row from a table
    pub fn delete(&mut self, table_name: &str, key: &Key) -> Result<()> {
        let table = self.get_table_mut(table_name)?;
        table.delete(key)?;
        self.save()
    }
}

// Re-export Row from types module
use crate::types::Row;

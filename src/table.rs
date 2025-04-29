use crate::btree::BTree;
use crate::error::{DbError, Result};
use crate::types::{Column, Key, Row, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A table in the database that stores rows of data
///
/// Tables are the primary data structure in Oxi-DB. Each table has a name,
/// a set of columns with defined types, and an optional primary key column.
/// Data is stored as rows, with each row identified by a unique key.
///
/// # Examples
///
/// Creating a new table:
///
/// ```
/// use oxi_db::{Column, ColumnType, Table};
///
/// let columns = vec![
///     Column::new("id", ColumnType::Integer),
///     Column::new("name", ColumnType::Text),
///     Column::new("active", ColumnType::Boolean),
/// ];
///
/// let table = Table::new("users", columns, Some("id".to_string()));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    /// The name of the table
    pub name: String,
    /// The columns in the table, defining the schema
    pub columns: Vec<Column>,
    /// The name of the primary key column, if any
    pub primary_key: Option<String>,
    /// The data stored in the table, organized as a B-tree
    data: BTree<Key, Row>,
}

impl Table {
    /// Create a new table with the given name and columns
    pub fn new(name: impl Into<String>, columns: Vec<Column>, primary_key: Option<String>) -> Self {
        Self {
            name: name.into(),
            columns,
            primary_key,
            data: BTree::new(),
        }
    }

    /// Insert a row into the table
    pub fn insert(&mut self, key: impl Into<Key>, values: Vec<Value>) -> Result<()> {
        let key = key.into();

        // Check if key already exists
        if self.data.search(&key).is_some() {
            return Err(DbError::KeyExists);
        }

        // Check if values match column count
        if values.len() != self.columns.len() {
            return Err(DbError::Other(format!(
                "Expected {} values, got {}",
                self.columns.len(),
                values.len()
            )));
        }

        // Validate value types
        for (i, value) in values.iter().enumerate() {
            self.validate_value_type(i, value)?;
        }

        let row = Row::new(values);
        self.data.insert(key, row);

        Ok(())
    }

    /// Validate that a value matches the expected column type
    fn validate_value_type(&self, column_idx: usize, value: &Value) -> Result<()> {
        use crate::types::ColumnType;

        let column = &self.columns[column_idx];
        let valid = match (&column.column_type, value) {
            (ColumnType::Integer, Value::Integer(_)) => true,
            (ColumnType::Float, Value::Float(_)) => true,
            (ColumnType::Text, Value::Text(_)) => true,
            (ColumnType::Boolean, Value::Boolean(_)) => true,
            (ColumnType::Blob, Value::Blob(_)) => true,
            (_, Value::Null) => true, // Allow NULL for any type
            _ => false,
        };

        if valid {
            Ok(())
        } else {
            Err(DbError::TypeConversionError)
        }
    }

    /// Get a row by key
    pub fn get(&self, key: &Key) -> Result<&Row> {
        self.data
            .search(key)
            .ok_or(DbError::KeyNotFound)
    }

    /// Update a row by key
    pub fn update(&mut self, key: &Key, values: Vec<Value>) -> Result<()> {
        // Check if key exists
        if self.data.search(key).is_none() {
            return Err(DbError::KeyNotFound);
        }

        // Check if values match column count
        if values.len() != self.columns.len() {
            return Err(DbError::Other(format!(
                "Expected {} values, got {}",
                self.columns.len(),
                values.len()
            )));
        }

        // Validate value types
        for (i, value) in values.iter().enumerate() {
            self.validate_value_type(i, value)?;
        }

        let row = Row::new(values);

        // Get mutable reference and update
        if let Some(existing_row) = self.data.get_mut(key) {
            *existing_row = row;
            Ok(())
        } else {
            Err(DbError::KeyNotFound)
        }
    }

    /// Delete a row by key
    pub fn delete(&mut self, key: &Key) -> Result<()> {
        self.data
            .remove(key)
            .ok_or(DbError::KeyNotFound)
            .map(|_| ())
    }

    /// Get all rows in the table
    pub fn get_all(&self) -> Vec<(Key, Row)> {
        self.data.to_vec()
    }

    /// Get the number of rows in the table
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the table is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a row as a map of column names to values
    pub fn get_as_map(&self, key: &Key) -> Result<HashMap<String, Value>> {
        let row = self.get(key)?;
        let mut map = HashMap::new();

        for (i, column) in self.columns.iter().enumerate() {
            if i < row.values.len() {
                map.insert(column.name.clone(), row.values[i].clone());
            }
        }

        Ok(map)
    }

    /// Find rows that match a predicate
    pub fn find<F>(&self, predicate: F) -> Vec<(Key, Row)>
    where
        F: Fn(&Row) -> bool,
    {
        let mut results = Vec::new();

        self.data.traverse(|k, v| {
            if predicate(v) {
                results.push((k.clone(), v.clone()));
            }
        });

        results
    }
}

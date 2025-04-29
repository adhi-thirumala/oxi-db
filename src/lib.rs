/*!
# Oxi-DB

Oxi-DB is a simple embedded database for Rust applications. It provides:

- Key-value storage with B-tree indexing
- Table-based data organization
- Support for multiple data types
- Persistence to disk
- Simple and intuitive API

## Example

```rust
use oxi_db::{Column, ColumnType, Database, Value};

// Create a new database
let mut db = Database::new("example.db");

// Create a table with columns
let columns = vec![
    Column::new("id", ColumnType::Integer),
    Column::new("name", ColumnType::Text),
    Column::new("active", ColumnType::Boolean),
];

db.create_table("users", columns, Some("id".to_string())).unwrap();

// Insert data
db.insert(
    "users",
    "1",
    vec![
        Value::Integer(1),
        Value::Text("Alice".to_string()),
        Value::Boolean(true),
    ],
).unwrap();

// Save the database
db.save().unwrap();
```
*/

mod btree;
mod database;
mod error;
mod table;
mod types;

// Re-export public items
pub use btree::BTree;
pub use database::Database;
pub use error::{DbError, Result};
pub use table::Table;
pub use types::{Column, ColumnType, Key, Row, Value};

/// Current version of the Oxi-DB crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Provides a simple example of how to use the Oxi-DB library
///
/// This function demonstrates:
/// - Creating a database
/// - Creating a table with columns
/// - Inserting data
/// - Saving the database
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
///
/// # Errors
///
/// Will return an error if any database operation fails
pub fn example() -> Result<()> {
    // Create a new database
    let mut db = Database::new("example.db");

    // Create a table with columns
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];

    db.create_table("users", columns, Some("id".to_string()))?;

    // Insert data
    db.insert(
        "users",
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    )?;

    db.insert(
        "users",
        "2",
        vec![
            Value::Integer(2),
            Value::Text("Bob".to_string()),
            Value::Boolean(false),
        ],
    )?;

    // Save the database
    db.save()?;

    Ok(())
}

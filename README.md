# Oxi-DB

A simple embedded database for Rust applications.

## Features

- Key-value storage with B-tree indexing
- Table-based data organization
- Support for multiple data types (Integer, Float, Text, Boolean, Blob)
- Persistence to disk
- Simple and intuitive API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
oxi-db = "0.1.0"
```

## Usage

### Creating a Database

```rust
use oxi_db::{Column, ColumnType, Database, Value};

// Create a new database
let mut db = Database::new("my_database.db");

// Create a table with columns
let columns = vec![
    Column::new("id", ColumnType::Integer),
    Column::new("name", ColumnType::Text),
    Column::new("email", ColumnType::Text),
    Column::new("active", ColumnType::Boolean),
];

db.create_table("users", columns, Some("id".to_string())).unwrap();

// Save the database
db.save().unwrap();
```

### Inserting Data

```rust
use oxi_db::{Database, Value};

let mut db = Database::open("my_database.db").unwrap();

// Insert a row
db.insert(
    "users",
    "1",  // key
    vec![
        Value::Integer(1),
        Value::Text("John Doe".to_string()),
        Value::Text("john@example.com".to_string()),
        Value::Boolean(true),
    ],
).unwrap();

// Save changes
db.save().unwrap();
```

### Querying Data

```rust
use oxi_db::{Database, Key};

let db = Database::open("my_database.db").unwrap();

// Get a row by key
let key = Key::from("1");
let row = db.get("users", &key).unwrap();

println!("User ID: {}", match &row.values[0] {
    Value::Integer(id) => id,
    _ => &0,
});

println!("Name: {}", match &row.values[1] {
    Value::Text(name) => name,
    _ => "Unknown",
});
```

### Updating Data

```rust
use oxi_db::{Database, Key, Value};

let mut db = Database::open("my_database.db").unwrap();

// Update a row
let key = Key::from("1");
db.update(
    "users",
    &key,
    vec![
        Value::Integer(1),
        Value::Text("John Smith".to_string()),
        Value::Text("john.smith@example.com".to_string()),
        Value::Boolean(true),
    ],
).unwrap();

// Save changes
db.save().unwrap();
```

### Deleting Data

```rust
use oxi_db::{Database, Key};

let mut db = Database::open("my_database.db").unwrap();

// Delete a row
let key = Key::from("1");
db.delete("users", &key).unwrap();

// Save changes
db.save().unwrap();
```

### Getting All Rows

```rust
use oxi_db::Database;

let db = Database::open("my_database.db").unwrap();

// Get all rows from a table
let table = db.get_table("users").unwrap();
for (key, row) in table.get_all() {
    println!("Key: {}", key);
    println!("Values: {:?}", row.values);
}
```

### Finding Rows

```rust
use oxi_db::{Database, Value};

let db = Database::open("my_database.db").unwrap();

// Find active users
let table = db.get_table("users").unwrap();
let active_users = table.find(|row| {
    if let Value::Boolean(active) = row.values[3] {
        active
    } else {
        false
    }
});

for (key, row) in active_users {
    println!("Active user: {}", match &row.values[1] {
        Value::Text(name) => name,
        _ => "Unknown",
    });
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

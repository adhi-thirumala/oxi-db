use oxi_db::{Column, ColumnType, Database, Key, Value};
use std::fs;
use std::path::Path;

// Helper function to create a test database
fn create_test_db(db_name: &str) -> Database {
    let db_path = format!("{}.db", db_name);

    // Remove the database file if it exists
    if Path::new(&db_path).exists() {
        fs::remove_file(&db_path).unwrap();
    }

    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(&db_path).parent() {
        fs::create_dir_all(parent).unwrap_or(());
    }

    let mut db = Database::new(&db_path);

    // Create a table
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];

    db.create_table("users", columns, Some("id".to_string())).unwrap();

    // Save the database to disk
    db.save().unwrap();

    db
}

#[test]
fn test_database_create_table() {
    let mut db = create_test_db("test_create_table");

    // Create another table
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("title", ColumnType::Text),
        Column::new("content", ColumnType::Text),
    ];

    let result = db.create_table("posts", columns, Some("id".to_string()));
    assert!(result.is_ok());

    // Check that the table exists
    let tables = db.list_tables();
    assert!(tables.contains(&"users".to_string()));
    assert!(tables.contains(&"posts".to_string()));

    // Clean up
    fs::remove_file("test_create_table.db").unwrap_or(());
}

#[test]
fn test_database_drop_table() {
    let mut db = create_test_db("test_drop_table");

    // Drop the table
    let result = db.drop_table("users");
    assert!(result.is_ok());

    // Check that the table no longer exists
    let tables = db.list_tables();
    assert!(!tables.contains(&"users".to_string()));

    // Clean up
    fs::remove_file("test_drop_table.db").unwrap_or(());
}

#[test]
fn test_database_insert_and_get() {
    let mut db = create_test_db("test_insert_get");

    // Insert a row
    let result = db.insert(
        "users",
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    );
    assert!(result.is_ok());

    // Get the row
    let key = Key::from("1");
    let row = db.get("users", &key);
    assert!(row.is_ok());

    let row = row.unwrap();
    assert_eq!(row.values[0], Value::Integer(1));
    assert_eq!(row.values[1], Value::Text("Alice".to_string()));
    assert_eq!(row.values[2], Value::Boolean(true));

    // Clean up
    fs::remove_file("test_insert_get.db").unwrap_or(());
}

#[test]
fn test_database_update() {
    let mut db = create_test_db("test_update");

    // Insert a row
    db.insert(
        "users",
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    )
    .unwrap();

    // Save the database
    db.save().unwrap();

    // Update the row
    let key = Key::from("1");
    let result = db.update(
        "users",
        &key,
        vec![
            Value::Integer(1),
            Value::Text("Alice Smith".to_string()),
            Value::Boolean(false),
        ],
    );
    assert!(result.is_ok());

    // Get the updated row
    let row = db.get("users", &key).unwrap();
    assert_eq!(row.values[0], Value::Integer(1));
    assert_eq!(row.values[1], Value::Text("Alice Smith".to_string()));
    assert_eq!(row.values[2], Value::Boolean(false));

    // Clean up
    fs::remove_file("test_update.db").unwrap_or(());
}

#[test]
fn test_database_delete() {
    let mut db = create_test_db("test_delete");

    // Insert a row
    db.insert(
        "users",
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    )
    .unwrap();

    // Save the database
    db.save().unwrap();

    // Delete the row
    let key = Key::from("1");
    let result = db.delete("users", &key);
    assert!(result.is_ok());

    // Try to get the deleted row
    let row = db.get("users", &key);
    assert!(row.is_err());

    // Clean up
    fs::remove_file("test_delete.db").unwrap_or(());
}

#[test]
fn test_database_save_and_open() {
    let mut db = create_test_db("test_save_open");

    // Insert a row
    db.insert(
        "users",
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    )
    .unwrap();

    // Save the database
    db.save().unwrap();

    // Open the database
    let db = Database::open("test_save_open.db").unwrap();

    // Check that the table exists
    let tables = db.list_tables();
    assert!(tables.contains(&"users".to_string()));

    // Check that the row exists
    let key = Key::from("1");
    let row = db.get("users", &key).unwrap();
    assert_eq!(row.values[0], Value::Integer(1));
    assert_eq!(row.values[1], Value::Text("Alice".to_string()));
    assert_eq!(row.values[2], Value::Boolean(true));

    // Clean up
    fs::remove_file("test_save_open.db").unwrap_or(());
}

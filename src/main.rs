#![cfg(feature = "cli")]

use oxi_db::{Column, ColumnType, Database, Key, Value};
use std::fs;
use std::path::Path;

fn main() {
  // Clean up any existing database file
  let db_path = "example.db";
  if Path::new(db_path).exists() {
    fs::remove_file(db_path).unwrap_or_else(|e| {
      println!("Warning: Could not remove existing database file: {}", e);
    });
  }

  // Create a new database
  let mut db = Database::new(db_path);
  println!("Created new database at '{}'", db_path);

  // Create a table with columns
  let columns = vec![
    Column::new("id", ColumnType::Integer),
    Column::new("name", ColumnType::Text),
    Column::new("email", ColumnType::Text),
    Column::new("active", ColumnType::Boolean),
  ];

  match db.create_table("users", columns, Some("id".to_string())) {
    Ok(_) => println!("Table 'users' created successfully"),
    Err(e) => println!("Error creating table: {:?}", e),
  }

  // Insert data
  let users = vec![
    (
      "1",
      vec![
        Value::Integer(1),
        Value::Text("John Doe".to_string()),
        Value::Text("john@example.com".to_string()),
        Value::Boolean(true),
      ],
    ),
    (
      "2",
      vec![
        Value::Integer(2),
        Value::Text("Jane Smith".to_string()),
        Value::Text("jane@example.com".to_string()),
        Value::Boolean(true),
      ],
    ),
    (
      "3",
      vec![
        Value::Integer(3),
        Value::Text("Bob Johnson".to_string()),
        Value::Text("bob@example.com".to_string()),
        Value::Boolean(false),
      ],
    ),
  ];

  for (id, values) in users {
    match db.insert("users", id, values) {
      Ok(_) => println!("Record with ID {} inserted successfully", id),
      Err(e) => println!("Error inserting record: {:?}", e),
    }
  }

  // Save the database
  match db.save() {
    Ok(_) => println!("Database saved successfully"),
    Err(e) => println!("Error saving database: {:?}", e),
  }

  // Query data
  println!("\nQuerying database:");
  let key = Key::from("2");
  match db.get("users", &key) {
    Ok(row) => {
      println!("Found user with ID 2:");
      println!(
        "  Name: {}",
        match &row.values[1] {
          Value::Text(name) => name,
          _ => "Unknown",
        }
      );
      println!(
        "  Email: {}",
        match &row.values[2] {
          Value::Text(email) => email,
          _ => "Unknown",
        }
      );
      println!(
        "  Active: {}",
        match &row.values[3] {
          Value::Boolean(active) => *active,
          _ => false,
        }
      );
    }
    Err(e) => println!("Error retrieving record: {:?}", e),
  }

  // Update data
  println!("\nUpdating user with ID 3:");
  let key = Key::from("3");
  match db.update(
    "users",
    &key,
    vec![
      Value::Integer(3),
      Value::Text("Robert Johnson".to_string()),
      Value::Text("robert@example.com".to_string()),
      Value::Boolean(true),
    ],
  ) {
    Ok(_) => println!("Record updated successfully"),
    Err(e) => println!("Error updating record: {:?}", e),
  }

  // Save changes
  db.save().unwrap();

  // List all tables
  println!("\nAvailable tables:");
  for table in db.list_tables() {
    println!("- {}", table);
  }

  // Get all users
  println!("\nAll users:");
  let table = db.get_table("users").unwrap();
  for (_key, row) in table.get_all() {
    let id = match row.values[0] {
      Value::Integer(id) => id.to_string(),
      _ => "Unknown".to_string(),
    };

    let name = match &row.values[1] {
      Value::Text(name) => name.clone(),
      _ => "Unknown".to_string(),
    };

    let active = match row.values[3] {
      Value::Boolean(active) => active,
      _ => false,
    };

    println!("User {}: {} (Active: {})", id, name, active);
  }

  println!("\nOxi-DB example completed!");
}

use oxi_db::{Column, ColumnType, Key, Table, Value};

#[test]
fn test_table_create() {
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];
    
    let table = Table::new("users", columns, Some("id".to_string()));
    
    assert_eq!(table.name, "users");
    assert_eq!(table.columns.len(), 3);
    assert_eq!(table.primary_key, Some("id".to_string()));
    assert!(table.is_empty());
}

#[test]
fn test_table_insert_and_get() {
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];
    
    let mut table = Table::new("users", columns, Some("id".to_string()));
    
    // Insert a row
    let result = table.insert(
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
    let row = table.get(&key);
    assert!(row.is_ok());
    
    let row = row.unwrap();
    assert_eq!(row.values[0], Value::Integer(1));
    assert_eq!(row.values[1], Value::Text("Alice".to_string()));
    assert_eq!(row.values[2], Value::Boolean(true));
}

#[test]
fn test_table_update() {
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];
    
    let mut table = Table::new("users", columns, Some("id".to_string()));
    
    // Insert a row
    table.insert(
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    ).unwrap();
    
    // Update the row
    let key = Key::from("1");
    let result = table.update(
        &key,
        vec![
            Value::Integer(1),
            Value::Text("Alice Smith".to_string()),
            Value::Boolean(false),
        ],
    );
    assert!(result.is_ok());
    
    // Get the updated row
    let row = table.get(&key).unwrap();
    assert_eq!(row.values[0], Value::Integer(1));
    assert_eq!(row.values[1], Value::Text("Alice Smith".to_string()));
    assert_eq!(row.values[2], Value::Boolean(false));
}

#[test]
fn test_table_delete() {
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];
    
    let mut table = Table::new("users", columns, Some("id".to_string()));
    
    // Insert a row
    table.insert(
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    ).unwrap();
    
    // Delete the row
    let key = Key::from("1");
    let result = table.delete(&key);
    assert!(result.is_ok());
    
    // Try to get the deleted row
    let row = table.get(&key);
    assert!(row.is_err());
}

#[test]
fn test_table_get_all() {
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];
    
    let mut table = Table::new("users", columns, Some("id".to_string()));
    
    // Insert rows
    table.insert(
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    ).unwrap();
    
    table.insert(
        "2",
        vec![
            Value::Integer(2),
            Value::Text("Bob".to_string()),
            Value::Boolean(false),
        ],
    ).unwrap();
    
    // Get all rows
    let rows = table.get_all();
    assert_eq!(rows.len(), 2);
    
    // Sort by key to ensure consistent order
    let mut rows = rows;
    rows.sort_by(|a, b| a.0.0.cmp(&b.0.0));
    
    assert_eq!(rows[0].0.0, "1");
    assert_eq!(rows[1].0.0, "2");
    
    assert_eq!(rows[0].1.values[0], Value::Integer(1));
    assert_eq!(rows[0].1.values[1], Value::Text("Alice".to_string()));
    assert_eq!(rows[0].1.values[2], Value::Boolean(true));
    
    assert_eq!(rows[1].1.values[0], Value::Integer(2));
    assert_eq!(rows[1].1.values[1], Value::Text("Bob".to_string()));
    assert_eq!(rows[1].1.values[2], Value::Boolean(false));
}

#[test]
fn test_table_get_as_map() {
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];
    
    let mut table = Table::new("users", columns, Some("id".to_string()));
    
    // Insert a row
    table.insert(
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    ).unwrap();
    
    // Get the row as a map
    let key = Key::from("1");
    let map = table.get_as_map(&key).unwrap();
    
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("id").unwrap(), &Value::Integer(1));
    assert_eq!(map.get("name").unwrap(), &Value::Text("Alice".to_string()));
    assert_eq!(map.get("active").unwrap(), &Value::Boolean(true));
}

#[test]
fn test_table_find() {
    let columns = vec![
        Column::new("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("active", ColumnType::Boolean),
    ];
    
    let mut table = Table::new("users", columns, Some("id".to_string()));
    
    // Insert rows
    table.insert(
        "1",
        vec![
            Value::Integer(1),
            Value::Text("Alice".to_string()),
            Value::Boolean(true),
        ],
    ).unwrap();
    
    table.insert(
        "2",
        vec![
            Value::Integer(2),
            Value::Text("Bob".to_string()),
            Value::Boolean(false),
        ],
    ).unwrap();
    
    // Find active users
    let active_users = table.find(|row| {
        if let Value::Boolean(active) = row.values[2] {
            active
        } else {
            false
        }
    });
    
    assert_eq!(active_users.len(), 1);
    assert_eq!(active_users[0].0.0, "1");
    assert_eq!(active_users[0].1.values[1], Value::Text("Alice".to_string()));
}

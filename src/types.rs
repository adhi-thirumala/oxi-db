use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported data types in the database
///
/// The `Value` enum represents all possible data types that can be stored in
/// the database. It includes support for NULL values, integers, floating-point
/// numbers, text strings, booleans, and binary data (blobs).
///
/// # Examples
///
/// Creating values of different types:
///
/// ```
/// use oxi_db::Value;
///
/// let null_value = Value::Null;
/// let integer_value = Value::Integer(42);
/// let float_value = Value::Float(3.14);
/// let text_value = Value::Text("Hello, world!".to_string());
/// let boolean_value = Value::Boolean(true);
/// let blob_value = Value::Blob(vec![0, 1, 2, 3, 4]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    /// Represents a NULL value (absence of value)
    Null,
    /// Represents a 64-bit signed integer
    Integer(i64),
    /// Represents a 64-bit floating-point number
    Float(f64),
    /// Represents a UTF-8 encoded string
    Text(String),
    /// Represents a boolean value (true or false)
    Boolean(bool),
    /// Represents binary data as a byte array
    Blob(Vec<u8>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Text(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Blob(b) => write!(f, "<BLOB: {} bytes>", b.len()),
        }
    }
}

/// Type for database keys
///
/// A `Key` is a wrapper around a string that uniquely identifies a row in a table.
/// Keys are used to insert, retrieve, update, and delete rows.
///
/// # Examples
///
/// Creating keys:
///
/// ```
/// use oxi_db::Key;
///
/// // From a string literal
/// let key1 = Key::from("user_1");
///
/// // From a String
/// let key2 = Key::from(String::from("user_2"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(pub String);

impl From<&str> for Key {
    /// Create a Key from a string slice
    fn from(s: &str) -> Self {
        Key(s.to_string())
    }
}

impl From<String> for Key {
    /// Create a Key from a String
    fn from(s: String) -> Self {
        Key(s)
    }
}

impl fmt::Display for Key {
    /// Format the Key as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A row in a table is a collection of values
///
/// A `Row` represents a single record in a table. It contains a vector of values,
/// where each value corresponds to a column in the table.
///
/// # Examples
///
/// Creating a row:
///
/// ```
/// use oxi_db::{Row, Value};
///
/// let row = Row::new(vec![
///     Value::Integer(1),
///     Value::Text("John Doe".to_string()),
///     Value::Boolean(true),
/// ]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Row {
    /// The values in the row, corresponding to columns in the table
    pub values: Vec<Value>,
}

impl Row {
    /// Create a new row with the given values
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of values for the row
    ///
    /// # Returns
    ///
    /// A new `Row` instance
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}

/// Column definition with name and type
///
/// A `Column` represents a field in a table. It has a name and a type that
/// defines what kind of data can be stored in that column.
///
/// # Examples
///
/// Creating a column:
///
/// ```
/// use oxi_db::{Column, ColumnType};
///
/// let id_column = Column::new("id", ColumnType::Integer);
/// let name_column = Column::new("name", ColumnType::Text);
/// let active_column = Column::new("active", ColumnType::Boolean);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Column {
    /// The name of the column
    pub name: String,
    /// The data type of the column
    pub column_type: ColumnType,
}

impl Column {
    /// Create a new column with the given name and type
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column
    /// * `column_type` - The data type of the column
    ///
    /// # Returns
    ///
    /// A new `Column` instance
    pub fn new(name: impl Into<String>, column_type: ColumnType) -> Self {
        Self {
            name: name.into(),
            column_type,
        }
    }
}

/// Supported column types in the database
///
/// The `ColumnType` enum defines the possible data types that can be used
/// for columns in a table. Each column must have a defined type, which is
/// used to validate data when inserting or updating rows.
///
/// # Examples
///
/// Using column types:
///
/// ```
/// use oxi_db::{Column, ColumnType};
///
/// let columns = vec![
///     Column::new("id", ColumnType::Integer),
///     Column::new("name", ColumnType::Text),
///     Column::new("salary", ColumnType::Float),
///     Column::new("active", ColumnType::Boolean),
///     Column::new("photo", ColumnType::Blob),
/// ];
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColumnType {
    /// Integer type (i64)
    Integer,
    /// Floating-point type (f64)
    Float,
    /// Text string type (String)
    Text,
    /// Boolean type (bool)
    Boolean,
    /// Binary data type (`Vec<u8>`)
    Blob,
}

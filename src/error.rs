use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Table already exists")]
    TableExists,
    
    #[error("Table does not exist")]
    TableNotFound,
    
    #[error("Key already exists")]
    KeyExists,
    
    #[error("Key not found")]
    KeyNotFound,
    
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Bincode error: {0}")]
    BincodeError(#[from] bincode::Error),
    
    #[error("Type conversion error")]
    TypeConversionError,
    
    #[error("Database error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, DbError>;

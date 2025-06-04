use thiserror::Error;

/// Represents the various error types that can occur in the Tenet system.
///
/// This enum collects all possible error types and implements the `Error` trait
/// for better error handling and error messages.
#[derive(Error, Debug)]
pub enum TenetError {
    /// Error accessing data storage
    #[error("Data storage access failed")]
    IoError(#[from] std::io::Error),
    
    /// Error during data serialization or deserialization
    #[error("Serialization or Deserialization failed")]
    SerializationError(#[from] serde_json::Error),
    
    /// Error connecting to the database
    #[error("Database Connection Error")]
    DatabaseConnectionError(#[from] r2d2::Error),
    
    /// General database error
    #[error("Database Error")]
    DatabaseError(#[from] diesel::result::Error),
    
    /// Error hashing passwords
    #[error("Password hashing Error")]
    PasswordHashingError(#[from] argon2::Error),
    
    /// The requested resource was not found
    #[error("Not found")]
    NotFoundError,
}



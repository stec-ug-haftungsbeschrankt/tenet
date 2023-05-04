use thiserror::Error;

#[derive(Error, Debug)]
pub enum TenetError {
    #[error("Data storage access failed")]
    IoError(#[from] std::io::Error),
    #[error("Serialization or Deserialization failed")]
    SerializationError(#[from] serde_json::Error),
    #[error("Database Connection Error")]
    DatabaseConnectionError(#[from] r2d2::Error),
    #[error("Database Error")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Password hashing Error")]
    PasswordHashingError(#[from] argon2::Error),
    #[error("Not found")]
    NotFoundError,
}



use thiserror::Error;

use crate::postgresql::service_error::ServiceError;

#[derive(Error, Debug)]
pub enum TenetError {
    #[error("Data storage access failed")]
    IoError(#[from] std::io::Error),
    #[error("Serialization or Deserialization failed")]
    SerializationError(#[from] serde_json::Error),
    #[error("Database Error: {} - {}", .0.status_code, .0.message)]
    DatabaseError(ServiceError),
    #[error("Not found")]
    NotFoundError,
}



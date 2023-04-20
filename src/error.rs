use thiserror::Error;

#[derive(Error, Debug)]
pub enum TenetError {
    #[error("Data storage access failed")]
    IoError(#[from] std::io::Error),
    #[error("Serialization or Deserialization failed")]
    SerializationError(#[from] serde_json::Error),
    #[error("Not found")]
    NotFoundError
}

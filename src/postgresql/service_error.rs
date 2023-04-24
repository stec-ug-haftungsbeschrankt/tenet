use serde::Deserialize;
use std::fmt;
use diesel::result::Error as DieselError;


#[derive(Debug, Deserialize)]
pub struct ServiceError {
    pub status_code: u64,
    pub message: String
}


impl ServiceError {
    pub fn new(status_code: u64, message: String) -> Self {
        ServiceError {
            status_code,
            message
        }
    }
}


impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.status_code, self.message)
    }
}


impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        match error {
            DieselError::DatabaseError(_, err) => ServiceError::new(409, err.message().to_string()),
            DieselError::NotFound => ServiceError::new(404, "Record not found".to_string()),
            err => ServiceError::new(500, format!("Diesel error: {}", err)),
        }
    }
}


use std::str::FromStr;

use crate::{storage::Storage, postgresql::dbapplication::DbApplication, application_type::ApplicationType};


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Application {
    pub id: uuid::Uuid,
    pub application_type: ApplicationType,
    pub storage: Storage
}

impl From<&DbApplication> for Application {
    fn from(value: &DbApplication) -> Self {
        Application {
            id: value.id,
            application_type: ApplicationType::from_str(&value.application_type).unwrap(),
            storage: todo!() //value.storage_id
        }
    }
}

impl Application {
    pub fn new(storage: Storage, application_type: ApplicationType) -> Self {
        Application { id: uuid::Uuid::new_v4(), application_type , storage }
    }
}

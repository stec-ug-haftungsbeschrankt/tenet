use crate::storage::Storage;






#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Application {
    pub id: uuid::Uuid,
    pub application_type: ApplicationType,
    pub storage: Storage
}

#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum ApplicationType {
    Shop
}


impl Application {
    pub fn new(storage: Storage, application_type: ApplicationType) -> Self {
        Application { id: uuid::Uuid::new_v4(), application_type , storage }
    }
}

use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};

use crate::{postgresql::dbapplication::DbApplication, application_type::ApplicationType};


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Application {
    pub id: uuid::Uuid,
    pub application_type: ApplicationType,
    pub storage_id: Option<uuid::Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}

impl From<&DbApplication> for Application {
    fn from(value: &DbApplication) -> Self {
        Application {
            id: value.id,
            application_type: ApplicationType::from_str(&value.application_type).unwrap(),
            storage_id: value.storage_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            db_tenant_id: value.db_tenant_id
        }
    }
}

impl Application {
    pub fn new(application_type: ApplicationType, storage_id: uuid::Uuid, tenant_id: uuid::Uuid) -> Self {
        Application { 
            id: uuid::Uuid::new_v4(), 
            application_type, 
            storage_id: Some(storage_id),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
        }
    }
}

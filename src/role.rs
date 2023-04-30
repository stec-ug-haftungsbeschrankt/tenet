use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};

use crate::{postgresql::dbrole::DbRole, role_type::RoleType};


#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Role { 
    pub id: uuid::Uuid,
    pub role_type: RoleType,
    pub user_id: Option<uuid::Uuid>,
    pub application_id: Option<uuid::Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}


impl From<&DbRole> for Role {
    fn from(value: &DbRole) -> Self {
        Role {
            id: value.id,
            role_type: RoleType::from_str(&value.role_type).unwrap(),
            user_id: value.user_id,
            application_id: value.application_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            db_tenant_id: value.db_tenant_id
        }
    }
}

impl Role {
    pub fn new(role_type: RoleType, user_id: uuid::Uuid, application_id: uuid::Uuid, tenant_id: uuid::Uuid) -> Self {
        Role { 
            id: uuid::Uuid::new_v4(), 
            user_id: Some(user_id), 
            application_id: Some(application_id),
            role_type,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
         }
    }
}
use std::{str::FromStr, fmt::Display};

use chrono::{NaiveDateTime, Utc};

use crate::postgresql::dbuser::DbUser;


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub encryption_mode: EncryptionModes,
    pub email: String,
    pub email_verified: bool,
    pub full_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}


impl From<&DbUser> for User {
    fn from(value: &DbUser) -> Self {
        User { 
            id: value.id, 
            username: value.email.clone(), // We do not have a concept of username at the moment, we just use email
            password: value.password.clone(), 
            encryption_mode: EncryptionModes::from_str(&value.encryption_mode).unwrap(), 
            email: value.email.clone(), 
            email_verified: value.email_verified, 
            full_name: value.full_name.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            db_tenant_id: value.db_tenant_id,
        }
    }
}


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum EncryptionModes {
    Argon2
}


impl FromStr for EncryptionModes {
    type Err = ();

    fn from_str(input: &str) -> Result<EncryptionModes, Self::Err> {
        match input {
            "Argon2"  => Ok(EncryptionModes::Argon2),
            _      => Err(()),
        }
    }
}

impl Display for EncryptionModes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl User {
    pub fn new(username: String, full_name: String, password: String, encryption_mode: EncryptionModes, email: String, email_verified: bool, tenant_id: uuid::Uuid) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            username,
            password,
            encryption_mode,
            email,
            email_verified,
            full_name,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
        }
    }
}




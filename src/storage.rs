use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};

use crate::{storage_type::StorageType, postgresql::dbstorage::DbStorage};




#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Storage {
    pub id: uuid::Uuid,
    pub storage_type: StorageType,
    pub path: Option<String>,
    pub connection_string: Option<String>,
    pub schema: Option<String>,
    pub table_prefix: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}


impl From<&DbStorage> for Storage {
    fn from(value: &DbStorage) -> Self {
        Storage { 
            id: value.id, 
            storage_type: StorageType::from_str(&value.storage_type).unwrap(), 
            path: value.path.clone(), 
            connection_string: value.connection_string.clone(), 
            schema: value.schema.clone(), 
            table_prefix: value.table_prefix.clone(), 
            created_at: value.created_at, 
            updated_at: value.updated_at, 
            db_tenant_id: value.db_tenant_id 
        }
    }
}

impl Storage {
    pub fn new_json_file(path: impl Into<String>, tenant_id: uuid::Uuid) -> Self {
        Storage { 
            id: uuid::Uuid::new_v4(),
            storage_type: StorageType::JsonFile, 
            path: Some(path.into()), 
            connection_string: None, 
            schema: None, 
            table_prefix: None, 
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
        }
    }

    pub fn new_sqlite_database(path: impl Into<String>, tenant_id: uuid::Uuid) -> Self {
        Storage { 
            id: uuid::Uuid::new_v4(),
            storage_type: StorageType::SqliteDatabase, 
            path: Some(path.into()), 
            connection_string: None, 
            schema: None, 
            table_prefix: None, 
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
        }
    }

    pub fn new_postgresql_database(connection_string: impl Into<String>, tenant_id: uuid::Uuid) -> Self {
        Storage { 
            id: uuid::Uuid::new_v4(),
            storage_type: StorageType::PostgreSqlDatabase, 
            path: None, 
            connection_string: Some(connection_string.into()), 
            schema: None, 
            table_prefix: None, 
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
        }
    }

    pub fn new_postgresql_schema(connection_string: impl Into<String>, schema: impl Into<String>, tenant_id: uuid::Uuid) -> Self {
        Storage { 
            id: uuid::Uuid::new_v4(),
            storage_type: StorageType::PostgreSqlSchema, 
            path: None, 
            connection_string: Some(connection_string.into()), 
            schema: Some(schema.into()), 
            table_prefix: None, 
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
        }
    }

    pub fn new_postgresql_table_prefix(connection_string: impl Into<String>, table_prefix: impl Into<String>, tenant_id: uuid::Uuid) -> Self {
        Storage { 
            id: uuid::Uuid::new_v4(),
            storage_type: StorageType::PostgreSqlTablePrefix, 
            path: None, 
            connection_string: Some(connection_string.into()), 
            schema: None, 
            table_prefix: Some(table_prefix.into()), 
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: Some(tenant_id)
        }
    }
}


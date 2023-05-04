use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{
    self,
    Queryable,
    Insertable,
};
use diesel::prelude::*;

use super::database;
use super::dbtenant::DbTenant;
use crate::TenetError;
use crate::schema::storages;


#[derive(Serialize, Deserialize, PartialEq, AsChangeset)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = storages)]
pub struct DbStorageMessage {
    pub storage_type: String,
    pub path: Option<String>,
    pub connection_string: Option<String>,
    pub schema: Option<String>,
    pub table_prefix: Option<String>,
    pub db_tenant_id: Option<uuid::Uuid>
}


#[derive(Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Queryable, Insertable)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = storages)]
pub struct DbStorage {
    pub id: uuid::Uuid,
    pub storage_type: String,
    pub path: Option<String>,
    pub connection_string: Option<String>,
    pub schema: Option<String>,
    pub table_prefix: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}

impl From<DbStorageMessage> for DbStorage {
    fn from(storage: DbStorageMessage) -> Self {
        DbStorage { 
            id: uuid::Uuid::new_v4(), 
            storage_type: storage.storage_type, 
            path: storage.path, 
            connection_string: storage.connection_string, 
            schema: storage.schema, 
            table_prefix: storage.table_prefix, 
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: storage.db_tenant_id
        }
    }
}


impl DbStorage {
    pub fn find_all() -> Result<Vec<Self>, TenetError> {
        let mut connection = database::connection()?;
        let storages = storages::table.load::<DbStorage>(&mut connection)?;
        Ok(storages)
    }

    pub fn find_by_tenant(id: Uuid) -> Result<Vec<Self>, TenetError> {
        let mut connection = database::connection()?;
        let storages = storages::table.filter(storages::db_tenant_id.eq(id)).load(&mut connection)?;
        Ok(storages)
    }

    pub fn find(tenant_id: uuid::Uuid, id: Uuid) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;
        let storage = storages::table
            .filter(storages::id.eq(id))
            .filter(storages::db_tenant_id.eq(tenant_id))
            .first(&mut connection)?;
        Ok(storage)
    }

    pub fn create(storage: DbStorageMessage) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;

        let new_storage = DbStorage::from(storage);

        let db_storage = diesel::insert_into(storages::table)
            .values(new_storage)
            .get_result(&mut connection)?;
        Ok(db_storage)
    }

    pub fn update(id: Uuid, storage: DbStorageMessage) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;

        let updated_storage = diesel::update(storages::table)
            .filter(storages::id.eq(id))
            .set(storage)
            .get_result(&mut connection)?;
        Ok(updated_storage)
    }

    pub fn delete(id: Uuid) -> Result<usize, TenetError> {
        let mut connection = database::connection()?;

        let result = diesel::delete(
            storages::table.filter(storages::id.eq(id))
            )
            .execute(&mut connection)?;
        Ok(result)
    }
}
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{
    self,
    Queryable,
    Insertable,
};
use diesel::prelude::*;

use super::service_error::ServiceError;
use super::database;
use super::dbtenant::DbTenant;
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


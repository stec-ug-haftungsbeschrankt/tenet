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
use crate::schema::roles;





#[derive(Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Queryable, Insertable)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = roles)]
pub struct DbRole {
    pub id: uuid::Uuid,
    pub role_type: String,
    pub user_id: Option<uuid::Uuid>,
    pub application_id: Option<uuid::Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}


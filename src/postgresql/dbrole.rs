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


#[derive(Debug, Serialize, Deserialize, PartialEq, AsChangeset)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = roles)]
pub struct DbRoleMessage {
    pub role_type: String,
    pub user_id: Option<uuid::Uuid>,
    pub application_id: Option<uuid::Uuid>,
    pub db_tenant_id: Option<uuid::Uuid>
}



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




impl From<DbRoleMessage> for DbRole {
    fn from(role: DbRoleMessage) -> Self {
        DbRole {
            id: Uuid::new_v4(),
            role_type: role.role_type,
            user_id: role.user_id,
            application_id: role.application_id,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: role.db_tenant_id
        }
    }
}



impl DbRole {
    pub fn find_all() -> Result<Vec<Self>, ServiceError> {
        let mut connection = database::connection()?;
        let roles = roles::table.load::<DbRole>(&mut connection)?;
        Ok(roles)
    }

    pub fn find_by_tenant(id: Uuid) -> Result<Vec<Self>, ServiceError> {
        let mut connection = database::connection()?;
        let roles = roles::table.filter(roles::db_tenant_id.eq(id)).load(&mut connection)?;
        Ok(roles)
    }

    pub fn find(id: Uuid) -> Result<Self, ServiceError> {
        let mut connection = database::connection()?;
        let role = roles::table.filter(roles::id.eq(id)).first(&mut connection)?;
        Ok(role)
    }

    pub fn create(role: DbRoleMessage) -> Result<Self, ServiceError> {
        let mut connection = database::connection()?;

        let new_role = DbRole::from(role);

        let db_role = diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(&mut connection)?;
        Ok(db_role)
    }

    pub fn update(id: Uuid, role: DbRoleMessage) -> Result<Self, ServiceError> {
        let mut connection = database::connection()?;

        let updated_role = diesel::update(roles::table)
            .filter(roles::id.eq(id))
            .set(role)
            .get_result(&mut connection)?;
        Ok(updated_role)
    }

    pub fn delete(id: Uuid) -> Result<usize, ServiceError> {
        let mut connection = database::connection()?;

        let result = diesel::delete(
            roles::table.filter(roles::id.eq(id))
            )
            .execute(&mut connection)?;
        Ok(result)
    }
}
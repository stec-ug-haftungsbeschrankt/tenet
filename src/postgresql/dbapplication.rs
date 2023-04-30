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
use crate::schema::applications;


#[derive(Debug, Serialize, Deserialize, PartialEq, AsChangeset)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = applications)]
pub struct DbApplicationMessage {
    pub application_type: String,
    pub storage_id: Option<uuid::Uuid>,
    pub db_tenant_id: Option<uuid::Uuid>
}



#[derive(Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Queryable, Insertable)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = applications)]
pub struct DbApplication {
    pub id: uuid::Uuid,
    pub application_type: String,
    pub storage_id: Option<uuid::Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}


impl From<DbApplicationMessage> for DbApplication {
    fn from(application: DbApplicationMessage) -> Self {
        DbApplication {
            id: Uuid::new_v4(),
            application_type: application.application_type,
            storage_id: application.storage_id,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: application.db_tenant_id
        }
    }
}


impl DbApplication {
    pub fn find_all() -> Result<Vec<Self>, TenetError> {
        let mut connection = database::connection()?;
        let applications = applications::table.load::<DbApplication>(&mut connection)?;
        Ok(applications)
    }

    pub fn find_by_tenant(id: Uuid) -> Result<Vec<Self>, TenetError> {
        let mut connection = database::connection()?;
        let applications = applications::table.filter(applications::db_tenant_id.eq(id)).load(&mut connection)?;
        Ok(applications)
    }

    pub fn find(tenant_id: uuid::Uuid, application_id: Uuid) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;
        let application = applications::table
            .filter(applications::id.eq(application_id))
            .filter(applications::db_tenant_id.eq(tenant_id))
            .first(&mut connection)?;
        Ok(application)
    }

    pub fn create(application: DbApplicationMessage) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;

        let new_application = DbApplication::from(application);

        let db_application = diesel::insert_into(applications::table)
            .values(new_application)
            .get_result(&mut connection)?;
        Ok(db_application)
    }

    pub fn update(id: Uuid, application: DbApplicationMessage) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;

        let updated_application = diesel::update(applications::table)
            .filter(applications::id.eq(id))
            .set(application)
            .get_result(&mut connection)?;
        Ok(updated_application)
    }

    pub fn delete(id: Uuid) -> Result<usize, TenetError> {
        let mut connection = database::connection()?;

        let result = diesel::delete(
            applications::table.filter(applications::id.eq(id))
            )
            .execute(&mut connection)?;
        Ok(result)
    }
}

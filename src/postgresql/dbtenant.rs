
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{
    self,
    Queryable,
    Insertable
};
use diesel::prelude::*;

use crate::TenetError;
use crate::schema::tenants;
use super::database;


#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = tenants)]
pub struct DbTenantMessage {
    pub title: String,
}


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = tenants)]
pub struct DbTenant {
    pub id: uuid::Uuid,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}


impl From<DbTenantMessage> for DbTenant {
    fn from(tenant: DbTenantMessage) -> Self {
        DbTenant {
            id: Uuid::new_v4(),
            title: tenant.title,
            created_at: Utc::now().naive_utc(),
            updated_at: None
        }
    }
}


impl DbTenant {
    pub fn find_all() -> Result<Vec<Self>, TenetError> {
        let mut connection = database::connection()?;
        let tenants = tenants::table.load::<DbTenant>(&mut connection)?;
        Ok(tenants)
    }

    pub fn find(id: Uuid) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;
        let tenant = tenants::table.filter(tenants::id.eq(id)).first(&mut connection)?;
        Ok(tenant)
    }

    pub fn find_by_title(title: String) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;
        let tenant = tenants::table.filter(tenants::title.eq(title)).first(&mut connection)?;
        Ok(tenant)
    }

    pub fn create(tenant: DbTenantMessage) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;

        let new_tenant = DbTenant::from(tenant);

        let db_tenant = diesel::insert_into(tenants::table)
            .values(new_tenant)
            .get_result(&mut connection)?;
        Ok(db_tenant)
    }

    pub fn update(id: Uuid, tenant: DbTenantMessage) -> Result<Self, TenetError> {
        let mut conn = database::connection()?;

        let db_tenant = diesel::update(tenants::table)
            .filter(tenants::id.eq(id))
            .set(tenant)
            .get_result(&mut conn)?;
        Ok(db_tenant)
    }

    pub fn delete(id: Uuid) -> Result<usize, TenetError> {
        let mut connection = database::connection()?;

        let res = diesel::delete(
                tenants::table.filter(tenants::id.eq(id))
            )
            .execute(&mut connection)?;
        Ok(res)
    }
}




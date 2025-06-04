use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{
    self,
    Queryable,
    Insertable,
};
use diesel::prelude::*;
use argon2::Config;
use rand::Rng;

use super::database;
use super::dbtenant::DbTenant;
use crate::TenetError;
use crate::schema::users;



#[derive(Serialize, Deserialize, PartialEq, AsChangeset)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = users)]
pub struct DbUserMessage {
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub encryption_mode: String,
    pub full_name: String,
    pub db_tenant_id: Option<uuid::Uuid>
}



#[derive(Debug, Serialize, Deserialize, Identifiable, Associations, PartialEq, Queryable, Insertable)]
#[diesel(belongs_to(DbTenant))]
#[diesel(table_name = users)]
pub struct DbUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub email_verified: bool,
    #[serde(skip_serializing)]
    pub password: String,
    pub encryption_mode: String,
    pub full_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub db_tenant_id: Option<uuid::Uuid>
}


impl From<DbUserMessage> for DbUser {
    fn from(user: DbUserMessage) -> Self {
        DbUser {
            id: Uuid::new_v4(),
            email: user.email,
            email_verified: user.email_verified,
            password: user.password,
            encryption_mode: user.encryption_mode,
            full_name: user.full_name,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            db_tenant_id: user.db_tenant_id
        }
    }
}

impl From<DbUser> for DbUserMessage {
    fn from(user: DbUser) -> Self {
        DbUserMessage {
            email: user.email,
            email_verified: user.email_verified,
            password: user.password,
            encryption_mode: user.encryption_mode,
            full_name: user.full_name,
            db_tenant_id: user.db_tenant_id
        }
    }
}


impl DbUser {
    pub fn find_by_tenant(tenant_id: Uuid) -> Result<Vec<Self>, TenetError> {
        let mut connection = database::connection()?;
        let users = users::table.filter(users::db_tenant_id.eq(tenant_id)).load(&mut connection)?;
        Ok(users)
    }

    pub fn find(tenant_id: uuid::Uuid, user_id: Uuid) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;
        let user = users::table
            .filter(users::id.eq(user_id))
            .filter(users::db_tenant_id.eq(tenant_id))
            .first(&mut connection)?;
        Ok(user)
    }

    pub fn find_by_email(email: String) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;
        let user = users::table
            .filter(users::email.eq(email))
            .first(&mut connection)?;
        Ok(user)
    }

    pub fn find_by_tenant_and_email(tenant_id: uuid::Uuid, email: String) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;
        let user = users::table
            .filter(users::db_tenant_id.eq(tenant_id))
            .filter(users::email.eq(email))
            .first(&mut connection)?;
        Ok(user)
    }

    pub fn create(user: DbUserMessage) -> Result<Self, TenetError> {
        let mut connection = database::connection()?;

        let mut new_user = DbUser::from(user);
        new_user.hash_password()?;

        let db_user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result(&mut connection)?;
        Ok(db_user)
    }

    pub fn update(id: Uuid, user: DbUserMessage) -> Result<Self, TenetError> {
        let mut conn = database::connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&mut conn)?;
        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, TenetError> {
        let mut conn = database::connection()?;

        let res = diesel::delete(
            users::table.filter(users::id.eq(id))
            )
            .execute(&mut conn)?;
        Ok(res)
    }

    fn hash_password(&mut self) -> Result<(), TenetError> {
        let salt: [u8; 32] = rand::rng().random();
        // Alternative would be the low_memory variant. Can be time consuming.
        // See https://github.com/sru-systems/rust-argon2/issues/52
        let config = Config::original(); 

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)?;

        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, TenetError> {
        Ok(argon2::verify_encoded(&self.password, password.as_bytes())?)
    }

}

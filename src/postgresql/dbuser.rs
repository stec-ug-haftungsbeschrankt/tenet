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

use super::service_error::ServiceError;
use super::database;
use super::dbtenant::DbTenant;
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
    pub db_tenant_id: uuid::Uuid
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
            db_tenant_id: Some(user.db_tenant_id)
        }
    }
}


impl DbUser {
    pub fn find_all() -> Result<Vec<Self>, ServiceError> {
        let mut connection = database::connection()?;
        let users = users::table.load::<DbUser>(&mut connection)?;
        Ok(users)
    }


    pub fn find_by_tenant(id: Uuid) -> Result<Vec<Self>, ServiceError> {
        let mut connection = database::connection()?;
        let users = users::table.filter(users::db_tenant_id.eq(id)).load(&mut connection)?;
        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ServiceError> {
        let mut connection = database::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&mut connection)?;
        Ok(user)
    }

    pub fn find_by_email(email: String) -> Result<Self, ServiceError> {
        let mut connection = database::connection()?;
        let user = users::table.filter(users::email.eq(email)).first(&mut connection)?;
        Ok(user)
    }

    pub fn create(user: DbUserMessage) -> Result<Self, ServiceError> {
        let mut connection = database::connection()?;

        let mut new_user = DbUser::from(user);
        new_user.hash_password()?;

        let db_user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result(&mut connection)?;
        Ok(db_user)
    }

    pub fn update(id: Uuid, user: DbUserMessage) -> Result<Self, ServiceError> {
        let mut conn = database::connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&mut conn)?;
        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ServiceError> {
        let mut conn = database::connection()?;

        let res = diesel::delete(
            users::table.filter(users::id.eq(id))
            )
            .execute(&mut conn)?;
        Ok(res)
    }

    fn hash_password(&mut self) -> Result<(), ServiceError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ServiceError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, ServiceError> {
        argon2::verify_encoded(&self.password, password.as_bytes())
            .map_err(|e| ServiceError::new(500, format!("Failed to verify password: {}", e)))
    }
}

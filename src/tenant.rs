use chrono::{Utc, NaiveDateTime};

use crate::{error::TenetError, application::Application, role::Role, user::User, postgresql::dbtenant::DbTenant};


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

impl From<DbTenant> for Tenant {
    fn from(value: DbTenant) -> Self {
        Tenant { 
            id: value.id, 
            title: value.title,
            created_at: value.created_at, 
            updated_at: value.updated_at 
        }
    }
}

impl Tenant {
    pub fn new(title: String) -> Self {
        Tenant { 
            id: uuid::Uuid::new_v4(), 
            title,
            created_at: Utc::now().naive_utc(),
            updated_at: None
        }
    }
/* 
    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn add_user(&mut self, user: User) -> Result<uuid::Uuid, TenetError>{
        self.users.push(user);
        self.updated_at = Some(Utc::now().naive_utc());

        Ok(self.users.last().unwrap().id)
    }

    pub fn remove_user(&mut self, user_id: uuid::Uuid) -> Result<User, TenetError> {
        if let Some(index) = self.users.iter().position(|u| u.id == user_id) {
            let user = self.users.remove(index);
            self.updated_at = Some(Utc::now().naive_utc());
            return Ok(user);
        }
        Err(TenetError::NotFoundError)
    }

    pub fn contains_username(&self, username: String) -> bool {
        self.users.iter().any(|u| u.username == username)
    }

    pub fn get_applications(&self) -> &Vec<Application> {
        &self.applications
    }

    pub fn add_application(&mut self, application: Application) -> Result<uuid::Uuid, TenetError> {
        self.applications.push(application);
        self.updated_at = Some(Utc::now().naive_utc());

        Ok(self.applications.last().unwrap().id)
    }

    pub fn remove_application(&mut self, application_id: uuid::Uuid) -> Result<Application, TenetError> {
        if let Some(index) = self.applications.iter().position(|a| a.id == application_id) {
            let application = self.applications.remove(index);
            self.updated_at = Some(Utc::now().naive_utc());
            return Ok(application);
        }
        Err(TenetError::NotFoundError)
    }

    pub fn get_roles(&self) -> &Vec<Role> {
        &self.roles
    }

    pub fn add_role(&mut self, role: Role) -> Result<uuid::Uuid, TenetError> {
        self.roles.push(role);
        self.updated_at = Some(Utc::now().naive_utc());

        Ok(self.roles.last().unwrap().id)
    }

    pub fn remove_role(&mut self, role_id: uuid::Uuid) -> Result<Role, TenetError> {
        if let Some(index) = self.roles.iter().position(|r| r.id == role_id) {
            let role = self.roles.remove(index);
            self.updated_at = Some(Utc::now().naive_utc());
            return Ok(role);
        }
        Err(TenetError::NotFoundError)
    }
    */
}
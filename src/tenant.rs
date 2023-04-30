use chrono::{Utc, NaiveDateTime};

use crate::{error::TenetError, user::User, postgresql::{dbtenant::DbTenant, dbuser::{DbUser, DbUserMessage}, dbapplication::{DbApplication, DbApplicationMessage}, dbrole::{DbRole, DbRoleMessage}}, Application, Role};


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

impl From<&DbTenant> for Tenant {
    fn from(value: &DbTenant) -> Self {
        Tenant { 
            id: value.id, 
            title: value.title.clone(),
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

    pub fn get_users(&self) -> Vec<User> {
        if let Ok(db_users) = DbUser::find_by_tenant(self.id) {
            return db_users.iter().map(|u| User::from(u)).collect();
        }
        Vec::new()
    }

    pub fn get_user_ids(&self) -> Vec<uuid::Uuid> {
        if let Ok(users) = DbUser::find_by_tenant(self.id) {
            return users.iter().map(|u| u.id).collect();
        }
        Vec::new()
    }

    pub fn add_user(&self, user: &User) -> Result<User, TenetError> {
        let user_message = DbUserMessage {
            email: user.email.clone(),
            email_verified: user.email_verified,
            password: user.password.clone(),
            encryption_mode: user.encryption_mode.to_string(),
            full_name: user.full_name.clone(),
            db_tenant_id: user.db_tenant_id
        };
        let created_user = DbUser::create(user_message)?;

        Ok(User::from(&created_user))
    }

    pub fn get_user_by_id(&self, user_id: uuid::Uuid) -> Result<User, TenetError> {
        let user = DbUser::find(self.id, user_id)?;
        Ok(User::from(&user))
    }

    pub fn delete_user(&self, user_id: uuid::Uuid) -> Result<(), TenetError> {
        DbUser::delete(user_id)?;
        Ok(())
    }

    pub fn contains_username(&self, username: String) -> bool {
        DbUser::find_by_tenant_and_email(self.id, username).is_ok()
    }


    pub fn get_applications(&self) -> Vec<Application> {
        if let Ok(applications) = DbApplication::find_by_tenant(self.id) {
            return applications.iter().map(|a| Application::from(a)).collect();
        }
        Vec::new()      
    }

    pub fn get_application_by_id(&self, application_id: uuid::Uuid) -> Result<Application, TenetError> {
        let application = DbApplication::find(self.id, application_id)?;
        Ok(Application::from(&application))
    }

    pub fn add_application(&self, application: Application) -> Result<Application, TenetError> {
        let application_message = DbApplicationMessage {
            application_type: application.application_type.to_string(),
            storage_id: application.storage_id,
            db_tenant_id: application.db_tenant_id
        };
        let created_application = DbApplication::create(application_message)?;

        Ok(Application::from(&created_application))
    }

    pub fn delete_application(&self, application_id: uuid::Uuid) -> Result<(), TenetError> {
        DbApplication::delete(application_id)?;
        Ok(())
    }

    pub fn get_roles(&self) -> Vec<Role> {
        if let Ok(roles) = DbRole::find_by_tenant(self.id) {
            return roles.iter().map(|r| Role::from(r)).collect();
        }
        Vec::new()
    }

    pub fn add_role(&mut self, role: Role) -> Result<Role, TenetError> {
        let role_message = DbRoleMessage {
            role_type: role.role_type.to_string(),
            user_id: role.user_id,
            application_id: role.application_id,
            db_tenant_id: role.db_tenant_id
        };
        let created_role = DbRole::create(role_message)?;

        Ok(Role::from(&created_role))
    }

    pub fn delete_role(&self, role_id: uuid::Uuid) -> Result<(), TenetError> {
        DbRole::delete(role_id)?;
        Ok(())
    }
}
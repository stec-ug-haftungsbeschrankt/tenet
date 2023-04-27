extern crate thiserror;
extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate log;

mod application;
mod error;
mod role;
mod storage;
mod tenant;
mod user;

mod schema;
mod postgresql;

use std::env;

pub use application::*;
pub use error::*;
use postgresql::{dbtenant::{DbTenant, DbTenantMessage}, database::initialize_database, dbuser::DbUser};
pub use role::*;
pub use storage::*;
pub use tenant::*;
pub use user::*;

use uuid::Uuid;

pub static DEFAULT_DATABASE_URL: &str = "postgres://postgres:@localhost/stec_tenet";

#[derive(Debug, Clone)]
pub struct Tenet { }


impl Tenet {
    pub fn new(connection_string: String) -> Self {
        env::set_var("TENET_DATABASE_URL", &connection_string);
        initialize_database();

        Tenet { }
    }


    pub fn get_tenant_ids(&self) -> Vec<Uuid> {
        if let Ok(tenants) = DbTenant::find_all() {
            return tenants.iter().map(|t| t.id).collect();
        }
        Vec::new()
    }

    pub fn get_tenant_id_by_username(&self, username: String) -> Result<uuid::Uuid, TenetError> {
        let user = DbUser::find_by_email(username).unwrap();
        if let Ok(tenant) = DbTenant::find(user.db_tenant_id.unwrap()) {
            return Ok(tenant.id);
        }

        Err(TenetError::NotFoundError)
    }

    pub fn get_tenant_by_username(&self, username: String) -> Result<Tenant, TenetError> {
        let user = DbUser::find_by_email(username).unwrap();
        if let Ok(tenant) = DbTenant::find(user.db_tenant_id.unwrap()) {
            return Ok(Tenant::from(&tenant));
        }

        Err(TenetError::NotFoundError)
    }

    pub fn get_tenant_by_id(&self, tenant_id: uuid::Uuid) -> Option<Tenant> {
        if let Ok(db_tenant) = DbTenant::find(tenant_id) {
            return Some(Tenant::from(&db_tenant));
        }
        None
    }

    pub fn create_tenant(&mut self, title: String) -> Result<Tenant, TenetError> {
        let tenant_message = DbTenantMessage {
            title
        };
        let created_tenant = DbTenant::create(tenant_message)?;

        Ok(Tenant::from(&created_tenant))
    }

    pub fn delete_tenant(&mut self, tenant_id: uuid::Uuid) -> Result<(), TenetError> {
        DbTenant::delete(tenant_id)?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crate::{application::{Application, ApplicationType}, role::{RoleType, Role}, user::{User, EncryptionModes}};

    use super::*;
/* 
    #[test]
    fn test() -> Result<(), TenetError> {
        let storage = Storage::JsonFile { path: "./test.json".to_string() };
        let mut tenet = Tenet::new(storage);

        let tenant = tenet.create_tenant()?;

        let user = User::new("someone@someplace.de".to_string(), "Danny Crane".to_string(), "password".to_string(), EncryptionModes::Argon2, "someone@someplace.de".to_string(), false);
        let user_id = tenant.add_user(user)?;

        let application_storage = Storage::JsonFile { path: "".to_string() };
        let application = Application::new(application_storage, ApplicationType::Shop);
        let application_id = tenant.add_application(application)?;

        let role = Role::new(user_id, application_id, RoleType::Administrator);
        tenant.add_role(role)?;

        let application_storage2 = Storage::JsonFile { path: "".to_string() };
        let application2 = Application::new(application_storage2, ApplicationType::Shop);
        tenant.add_application(application2)?;

        println!("{:?}", tenet);

        tenet.persist()
    }
    */


    #[test]
    fn create_tenant_test() {
        cleanup_database();
        
        let mut tenet = Tenet::new(DEFAULT_DATABASE_URL.to_string());

        let title: String = "SomeTenantTitle".to_string();
        let tenant = tenet.create_tenant(title.clone()).unwrap();
        assert_eq!(title, tenant.title);
    
        let loaded_tenant = tenet.get_tenant_by_id(tenant.id).unwrap();
        assert_eq!(title, loaded_tenant.title);
    }

    #[test]
    fn get_tenant_ids_test() {
        cleanup_database();

        let mut tenet = Tenet::new(DEFAULT_DATABASE_URL.to_string());

        let precondition = tenet.get_tenant_ids();
        assert_eq!(0, precondition.len(), "Table must be empty");

        let title: String = "SomeTenantTitle".to_string();
        let tenant_a = tenet.create_tenant(title.clone()).unwrap();
        let tenant_b = tenet.create_tenant(title.clone()).unwrap();

        let tenants = tenet.get_tenant_ids();

        assert_eq!(2, tenants.len());
        assert!(tenants.iter().any(|t| t == &tenant_a.id));
        assert!(tenants.iter().any(|t| t == &tenant_b.id));
    }

    #[test]
    fn create_user() {
        cleanup_database();

        let mut tenet = Tenet::new(DEFAULT_DATABASE_URL.to_string());

        let mut tenant = tenet.create_tenant("TenantTitle".to_string()).unwrap();

        let user = User::new(
            "someone@something.de".to_string(),
            "Danny Crane".to_string(), 
            "password".to_string(), 
            EncryptionModes::Argon2, 
            "someone@something.de".to_string(), 
            true, 
            tenant.id);

        let created_user = tenant.add_user(&user).unwrap();

        assert_eq!(user.username, created_user.username);
        assert_eq!(user.full_name, created_user.full_name);
        assert_eq!(user.encryption_mode, created_user.encryption_mode);
        assert_eq!(user.email, created_user.email);
        assert_eq!(user.email_verified, created_user.email_verified);
        assert_eq!(user.db_tenant_id, created_user.db_tenant_id);

        let get_user = tenant.get_user_by_id(created_user.id).unwrap();
        
        assert_eq!(created_user.id, get_user.id);
    }


    fn cleanup_database() {
        let mut tenet = Tenet::new(DEFAULT_DATABASE_URL.to_string());

        for tenant_id in tenet.get_tenant_ids() {

            let tenant = tenet.get_tenant_by_id(tenant_id).unwrap();

            let users = tenant.get_users();
            for user in users {
                tenant.delete_user(user.id);
            }

            tenet.delete_tenant(tenant_id);
        }
    }

}

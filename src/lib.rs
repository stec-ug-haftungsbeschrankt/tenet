extern crate thiserror;

mod application;
mod error;
mod role;
mod storage;
mod tenant;
mod user;

pub use application::*;
pub use error::*;
pub use role::*;
pub use storage::*;
pub use tenant::*;
pub use user::*;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Tenet {
    storage: Storage,
    tenants: Vec<Tenant>
 }


impl Tenet {
    pub fn new(storage: Storage) -> Self {
        let tenants = storage.read().unwrap();

        Tenet { 
            storage,
            tenants 
        }
    }

    pub fn persist(&self) -> Result<(), TenetError> {
        self.storage.write(&self.tenants)
    }

    fn get_tenant_ids(&self) -> Vec<Uuid> {
        self.tenants.iter().map(|t| t.id).collect()
    }

    pub fn get_tenant_id_by_username(&self, username: String) -> Result<uuid::Uuid, TenetError> {
        if let Some(tenant) = self.tenants.iter().find(|t| t.contains_username(username.clone())) {
            return Ok(tenant.id);
        }
        Err(TenetError::NotFoundError)
    }

    pub fn get_tenant_by_username(&self, username: String) -> Result<&Tenant, TenetError> {
        if let Some(tenant) = self.tenants.iter().find(|t| t.contains_username(username.clone())) {
            return Ok(tenant);
        }
        Err(TenetError::NotFoundError)
    }

    pub fn get_tenant_by_id(&self, tenant_id: uuid::Uuid) -> Option<&Tenant> {
        self.tenants.iter().find(|t| t.id == tenant_id)
    }

    pub fn create_tenant(&mut self) -> Result<&mut Tenant, TenetError> {
        let tenant = Tenant::new();
        self.tenants.push(tenant);
        
        Ok(self.tenants.last_mut().unwrap())
    }
}



#[cfg(test)]
mod tests {
    use crate::{application::{Application, ApplicationType}, role::{RoleType, Role}, user::{User, EncryptionModes}};

    use super::*;

    #[test]
    fn test() -> Result<(), TenetError> {
        let storage = Storage::JsonFile { path: "./test.json".to_string() };
        let mut tenet = Tenet::new(storage);

        let tenant = tenet.create_tenant()?;

        let user = User::new("someone@someplace.de".to_string(), "Danny Crane".to_string(), "password".to_string(), EncryptionModes::Argon2);
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

}

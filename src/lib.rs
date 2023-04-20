extern crate thiserror;

mod error;
mod storage;

use storage::Storage;

pub struct Tenet { 
    tenants: Vec<Tenant>,
    storage: Storage
}


impl Tenet {
    pub fn new(storage: Storage) -> Self {
        Tenet { tenants: Vec::new(), storage }
    }

    pub fn get_tenant_by_username(username: String) -> String {
        String::new()
    }
}



#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tenant {
    id: uuid::Uuid,
    users: Vec::<User>,
    applications: Vec<Application>,
    roles: Vec<Role>
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct User {
    id: uuid::Uuid,
    username: String,
    password: String,
    encryption_mode: EncryptionModes,
    full_name: String
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct Role {
    id: uuid::Uuid,
    user: User,
    application: Application,
    role: UserRole
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
enum UserRole {
    Administrator,
    User
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
enum EncryptionModes {
    Argon2
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct Application {
    id: uuid::Uuid,
    application_type: ApplicationType,
    storage: Storage
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
enum ApplicationType {
    Shop
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//! # Tenet
//! 
//! `tenet` is a library for managing tenants, applications, and users for SaaS applications.
//!
//! ## Main Features
//!
//! * Multi-tenant architecture for SaaS applications
//! * User management with secure password storage
//! * Role management for different permission levels
//! * Application configuration with various storage options
//!
//! ## Examples
//!
//! ```
//! use tenet::{Tenet, Tenant, User, encryption_modes::EncryptionModes};
//!
//! // Initialize the Tenet instance with a database connection
//! let tenet = Tenet::new("postgres://user:password@localhost/mydb".to_string());
//!
//! // Create a new tenant
//! let tenant = tenet.create_tenant("My Company".to_string()).unwrap();
//!
//! // Create a user and add it to the tenant
//! let user = User::new(
//!     "admin@example.com".to_string(),
//!     "Admin User".to_string(),
//!     "secure_password".to_string(),
//!     EncryptionModes::Argon2,
//!     "admin@example.com".to_string(),
//!     true,
//!     tenant.id
//! );
//! let created_user = tenant.add_user(&user).unwrap();
//! ```

extern crate thiserror;
extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate serde_derive;
extern crate log;

mod application;
pub mod application_type;
pub mod encryption_modes;
mod error;
mod role;
pub mod role_type;
mod storage;
pub mod storage_type;
mod tenant;
mod user;

mod schema;
mod postgresql;

use std::sync::OnceLock;
use log::info;
use postgresql::{dbtenant::{DbTenant, DbTenantMessage}, dbuser::DbUser};
use uuid::Uuid;

pub use application::*;
pub use error::*;
pub use role::*;
pub use storage::*;
pub use tenant::*;
pub use user::*;

/// Default database URL used when no connection string is provided.
pub static DEFAULT_DATABASE_URL: &str = "postgres://postgres:@localhost/stec_tenet";
/// Stores the connection string for the database connection.
static CONNECTION_STRING: OnceLock<String> = OnceLock::new();

/// Main structure for interacting with the Tenet system.
///
/// This structure is the primary entry point for working with the Tenet library.
/// It manages the database connection and provides methods for managing tenants.
///
/// # Example
///
/// ```
/// use tenet::Tenet;
///
/// // Initialize Tenet with a custom database connection
/// let tenet = Tenet::new("postgres://user:password@localhost/mydb".to_string());
///
/// // Create a new tenant
/// let tenant = tenet.create_tenant("My Company".to_string()).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Tenet { }


unsafe impl Send for Tenet {}
unsafe impl Sync for Tenet {}

impl Tenet {
    /// Creates a new Tenet instance with the specified database connection.
    ///
    /// # Parameters
    ///
    /// * `connection_string` - The connection string for the database.
    ///   If empty, the default URL will be used.
    ///
    /// # Example
    ///
    /// ```
    /// use tenet::Tenet;
    ///
    /// let tenet = Tenet::new("postgres://user:password@localhost/mydb".to_string());
    /// ```
    pub fn new(connection_string: String) -> Self {     
        let database_url = if connection_string.is_empty() {
            info!("Database url not set, using default ConnectionString");
            DEFAULT_DATABASE_URL.to_string()
        } else {
            connection_string
        };

        if let Err(e) = CONNECTION_STRING.set(database_url) {
            println!("ConnectionString {}", e);
        }

        //initialize_database();
        Tenet { }
    }

    /// Returns a list of all tenant IDs.
    ///
    /// # Returns
    ///
    /// A `Vec<Uuid>` containing the IDs of all tenants in the system.
    ///
    /// # Example
    ///
    /// ```
    /// use tenet::Tenet;
    ///
    /// let tenet = Tenet::new("postgres://user:password@localhost/mydb".to_string());
    /// let tenant_ids = tenet.get_tenant_ids();
    /// ```
    pub fn get_tenant_ids(&self) -> Vec<Uuid> {
        if let Ok(tenants) = DbTenant::find_all() {
            return tenants.iter().map(|t| t.id).collect();
        }
        Vec::new()
    }

    /// Finds a tenant ID for a user by their username.
    ///
    /// # Parameters
    ///
    /// * `username` - The username (typically the email address) of the user.
    ///
    /// # Returns
    ///
    /// A `Result` with the tenant ID as a `Uuid` or a `TenetError`.
    ///
    /// # Errors
    ///
    /// Returns a `TenetError::NotFoundError` if the user or tenant is not found.
    pub fn get_tenant_id_by_username(&self, username: String) -> Result<uuid::Uuid, TenetError> {
        let user = DbUser::find_by_email(username).unwrap();
        if let Ok(tenant) = DbTenant::find(user.db_tenant_id.unwrap()) {
            return Ok(tenant.id);
        }

        Err(TenetError::NotFoundError)
    }

    /// Finds a tenant by a username of one of its users.
    ///
    /// # Parameters
    ///
    /// * `username` - The username (typically the email address) of the user.
    ///
    /// # Returns
    ///
    /// A `Result` with the `Tenant` object or a `TenetError`.
    ///
    /// # Errors
    ///
    /// Returns a `TenetError::NotFoundError` if the user or tenant is not found.
    pub fn get_tenant_by_username(&self, username: String) -> Result<Tenant, TenetError> {
        let user = DbUser::find_by_email(username).unwrap();
        if let Ok(tenant) = DbTenant::find(user.db_tenant_id.unwrap()) {
            return Ok(Tenant::from(&tenant));
        }

        Err(TenetError::NotFoundError)
    }

    /// Finds a tenant by its ID.
    ///
    /// # Parameters
    ///
    /// * `tenant_id` - The ID of the tenant to find.
    ///
    /// # Returns
    ///
    /// An `Option<Tenant>` containing the tenant if found, otherwise `None`.
    pub fn get_tenant_by_id(&self, tenant_id: uuid::Uuid) -> Option<Tenant> {
        if let Ok(db_tenant) = DbTenant::find(tenant_id) {
            return Some(Tenant::from(&db_tenant));
        }
        None
    }

    /// Updates the title of a tenant.
    ///
    /// # Parameters
    ///
    /// * `tenant_id` - The ID of the tenant to update.
    /// * `title` - The new title for the tenant.
    ///
    /// # Returns
    ///
    /// A `Result` with the updated `Tenant` object or a `TenetError`.
    ///
    /// # Errors
    ///
    /// Returns a `TenetError` if the update fails.
    pub fn set_tenant_title(&self, tenant_id: uuid::Uuid, title: String) -> Result<Tenant, TenetError> {
        let tenant_message = DbTenantMessage {
            title
        };
        
        let updated_tenant = DbTenant::update(tenant_id, tenant_message)?;

        Ok(Tenant::from(&updated_tenant))
    }

    /// Creates a new tenant.
    ///
    /// # Parameters
    ///
    /// * `title` - The title for the new tenant.
    ///
    /// # Returns
    ///
    /// A `Result` with the newly created `Tenant` object or a `TenetError`.
    ///
    /// # Errors
    ///
    /// Returns a `TenetError` if the creation fails.
    ///
    /// # Example
    ///
    /// ```
    /// use tenet::Tenet;
    ///
    /// let tenet = Tenet::new("postgres://user:password@localhost/mydb".to_string());
    /// let tenant = tenet.create_tenant("My Company".to_string()).unwrap();
    /// ```
    pub fn create_tenant(&self, title: String) -> Result<Tenant, TenetError> {
        let tenant_message = DbTenantMessage {
            title
        };
        let created_tenant = DbTenant::create(tenant_message)?;

        Ok(Tenant::from(&created_tenant))
    }

    /// Deletes a tenant by its ID.
    ///
    /// # Parameters
    ///
    /// * `tenant_id` - The ID of the tenant to delete.
    ///
    /// # Returns
    ///
    /// A `Result` with `()` on success or a `TenetError`.
    ///
    /// # Errors
    ///
    /// Returns a `TenetError` if the deletion fails.
    pub fn delete_tenant(&self, tenant_id: uuid::Uuid) -> Result<(), TenetError> {
        DbTenant::delete(tenant_id)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::runners::SyncRunner;
    use crate::{application_type::ApplicationType, encryption_modes::EncryptionModes, role_type::RoleType, user::User};

    use super::*;

    fn test_harness(test_code: impl Fn(String)) {
        let node = Postgres::default().start().expect("Unable to start container");

        let host = node.get_host().unwrap();
        let port = node.get_host_port_ipv4(5432).unwrap();
        let connection_string = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);
        println!("{}", connection_string);
        test_code(connection_string);

        node.stop().expect("Failed to stop postgres container");
        //node.rm();
    }
    
    #[test]
    fn create_tenant_test() {
        test_harness(|connection_string| {
            let tenet = Tenet::new(connection_string);

            let title: String = "SomeTenantTitle".to_string();
            let tenant = tenet.create_tenant(title.clone()).unwrap();
            assert_eq!(title, tenant.title);

            let loaded_tenant = tenet.get_tenant_by_id(tenant.id).unwrap();
            assert_eq!(title, loaded_tenant.title);
        });
    }

    #[test]
    fn get_tenant_ids_test() {
        test_harness(|connection_string| {
            let tenet = Tenet::new(connection_string);

            let precondition = tenet.get_tenant_ids();
            assert_eq!(0, precondition.len(), "Table must be empty");

            let title: String = "SomeTenantTitle".to_string();
            let tenant_a = tenet.create_tenant(title.clone()).unwrap();
            let tenant_b = tenet.create_tenant(title.clone()).unwrap();

            let tenants = tenet.get_tenant_ids();

            assert_eq!(2, tenants.len());
            assert!(tenants.iter().any(|t| t == &tenant_a.id));
            assert!(tenants.iter().any(|t| t == &tenant_b.id));
        });
    }

    #[test]
    fn create_user() {
        test_harness(|connection_string| {
            let tenet = Tenet::new(connection_string);

            let tenant = tenet.create_tenant("TenantTitle".to_string()).unwrap();

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
        });
    }

    #[test]
    fn create_application() {
        test_harness(|connection_string| {
            let tenet = Tenet::new(connection_string);
        
            let tenant = tenet.create_tenant("TenantTitle".to_string()).unwrap();

            let storage = Storage::new_json_file("some_path", tenant.id);
            let storage = tenant.add_storage(&storage).unwrap();

            let application = Application::new(ApplicationType::Shop, storage.id, tenant.id);
            let created_application = tenant.add_application(&application).unwrap();

            assert_eq!(application.application_type, created_application.application_type);
            assert_eq!(application.storage_id, created_application.storage_id);
            assert_eq!(application.db_tenant_id, created_application.db_tenant_id);

            let get_application = tenant.get_application_by_id(created_application.id).unwrap();

            assert_eq!(created_application.id, get_application.id);
        });
    }

    #[test]
    fn create_role() {
        test_harness(|connection_string| {
            let tenet = Tenet::new(connection_string); 
            let tenant = tenet.create_tenant("TenantTitle".to_string()).unwrap();

            let storage = Storage::new_json_file("some_path", tenant.id);
            let storage = tenant.add_storage(&storage).unwrap();

            let application = Application::new(ApplicationType::Shop, storage.id, tenant.id);
            let created_application = tenant.add_application(&application).unwrap();

            let user = User::new(
                "someone@something.de".to_string(),
                "Danny Crane".to_string(), 
                "password".to_string(), 
                EncryptionModes::Argon2, 
                "someone@something.de".to_string(), 
                true, 
                tenant.id);
            let created_user = tenant.add_user(&user).unwrap();

            let role = Role::new(RoleType::Administrator, created_user.id, created_application.id, tenant.id);
            let created_role = tenant.add_role(&role).unwrap();

            assert_eq!(role.role_type, created_role.role_type);
            assert_eq!(role.user_id, created_role.user_id);
            assert_eq!(role.application_id, created_role.application_id);
            assert_eq!(role.db_tenant_id, created_role.db_tenant_id);

            let get_role = tenant.get_role_by_id(created_role.id).unwrap();

            assert_eq!(created_role.id, get_role.id);
        });
    }
}

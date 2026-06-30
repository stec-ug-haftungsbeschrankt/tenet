use chrono::{Utc, NaiveDateTime};

use crate::{
    error::TenetError,
    postgresql::database::Pool,
    user::User,
    postgresql::{dbtenant::DbTenant, dbuser::{DbUser, DbUserMessage},
    dbapplication::{DbApplication, DbApplicationMessage},
    dbrole::{DbRole, DbRoleMessage},
    dbstorage::{DbStorage, DbStorageMessage}},
    Application,
    Role,
    Storage
};


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip, default = "disconnected_pool")]
    pub(crate) pool: Pool
}

fn disconnected_pool() -> Pool {
    Pool::builder().build_unchecked(diesel::r2d2::ConnectionManager::new(""))
}

impl Tenant {
    pub(crate) fn from_db(value: &DbTenant, pool: Pool) -> Self {
        Tenant {
            id: value.id,
            title: value.title.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            pool
        }
    }
}

impl Tenant {
    /// Creates a transient, unsaved `Tenant` value not yet associated with any database.
    ///
    /// This value is not connected to a database, so calling any of `Tenant`'s
    /// methods that read or write data will fail. Obtain a usable `Tenant` from
    /// `Tenet::create_tenant`, `Tenet::get_tenant_by_id`, or similar lookup methods.
    pub fn new(title: String) -> Self {
        Tenant {
            id: uuid::Uuid::new_v4(),
            title,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            pool: disconnected_pool()
        }
    }

    /* Users */
    pub fn get_users(&self) -> Vec<User> {
        if let Ok(db_users) = DbUser::find_by_tenant(&self.pool, self.id) {
            return db_users.iter().map(User::from).collect();
        }
        Vec::new()
    }

    pub fn get_user_ids(&self) -> Vec<uuid::Uuid> {
        if let Ok(users) = DbUser::find_by_tenant(&self.pool, self.id) {
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
        let created_user = DbUser::create(&self.pool, user_message)?;

        Ok(User::from(&created_user))
    }

    pub fn get_user_by_id(&self, user_id: uuid::Uuid) -> Result<User, TenetError> {
        let user = DbUser::find(&self.pool, self.id, user_id)?;
        Ok(User::from(&user))
    }

    pub fn delete_user(&self, user_id: uuid::Uuid) -> Result<(), TenetError> {
        DbUser::delete(&self.pool, user_id)?;
        Ok(())
    }

    pub fn contains_username(&self, username: String) -> bool {
        DbUser::find_by_tenant_and_email(&self.pool, self.id, username).is_ok()
    }

    pub fn set_user_verified(&self, user_id: uuid::Uuid) -> bool {
        if let Ok(user) = DbUser::find(&self.pool, self.id, user_id) {
            let user_message = DbUserMessage::from(user);
            return DbUser::update(&self.pool, user_id, user_message).is_ok();
        }
        false
    }

    pub fn authenticate_user(&self, username: String, password: String) -> Option<User> {
        if let Ok(user) = DbUser::find_by_tenant_and_email(&self.pool, self.id, username) {
            if let Ok(verified) = user.verify_password(&password) {
                if verified {
                    return Some(User::from(&user))
                }
            }
        }
        None
    }

    /* Applications */
    pub fn get_applications(&self) -> Vec<Application> {
        if let Ok(applications) = DbApplication::find_by_tenant(&self.pool, self.id) {
            return applications.iter().map(Application::from).collect();
        }
        Vec::new()
    }

    pub fn get_application_by_id(&self, application_id: uuid::Uuid) -> Result<Application, TenetError> {
        let application = DbApplication::find(&self.pool, self.id, application_id)?;
        Ok(Application::from(&application))
    }

    pub fn add_application(&self, application: &Application) -> Result<Application, TenetError> {
        let application_message = DbApplicationMessage {
            application_type: application.application_type.to_string(),
            storage_id: application.storage_id,
            db_tenant_id: application.db_tenant_id
        };
        let created_application = DbApplication::create(&self.pool, application_message)?;

        Ok(Application::from(&created_application))
    }

    pub fn delete_application(&self, application_id: uuid::Uuid) -> Result<(), TenetError> {
        DbApplication::delete(&self.pool, application_id)?;
        Ok(())
    }

    /* Storage */
    pub fn get_storages(&self) -> Vec<Storage> {
        if let Ok(storages) = DbStorage::find_by_tenant(&self.pool, self.id) {
            return storages.iter().map(Storage::from).collect();
        }
        Vec::new()
    }

    pub fn get_storage_by_id(&self, storage_id: uuid::Uuid) -> Result<Storage, TenetError> {
        let storage = DbStorage::find(&self.pool, self.id, storage_id)?;
        Ok(Storage::from(&storage))
    }

    pub fn add_storage(&self, storage: &Storage) -> Result<Storage, TenetError> {
        let storage_message = DbStorageMessage {
            storage_type: storage.storage_type.to_string(),
            path: storage.path.clone(),
            connection_string: storage.connection_string.clone(),
            schema: storage.schema.clone(),
            table_prefix: storage.table_prefix.clone(),
            db_tenant_id: storage.db_tenant_id
        };
        let created_storage = DbStorage::create(&self.pool, storage_message)?;

        Ok(Storage::from(&created_storage))
    }

    pub fn delete_storage(&self, storage_id: uuid::Uuid) -> Result<(), TenetError> {
        DbStorage::delete(&self.pool, storage_id)?;
        Ok(())
    }

    /* Roles */
    pub fn get_roles(&self) -> Vec<Role> {
        if let Ok(roles) = DbRole::find_by_tenant(&self.pool, self.id) {
            return roles.iter().map(Role::from).collect();
        }
        Vec::new()
    }

    pub fn get_role_by_id(&self, role_id: uuid::Uuid) -> Result<Role, TenetError> {
        let role = DbRole::find(&self.pool, self.id, role_id)?;
        Ok(Role::from(&role))
    }

    pub fn add_role(&self, role: &Role) -> Result<Role, TenetError> {
        let role_message = DbRoleMessage {
            role_type: role.role_type.to_string(),
            user_id: role.user_id,
            application_id: role.application_id,
            db_tenant_id: role.db_tenant_id
        };
        let created_role = DbRole::create(&self.pool, role_message)?;

        Ok(Role::from(&created_role))
    }

    pub fn delete_role(&self, role_id: uuid::Uuid) -> Result<(), TenetError> {
        DbRole::delete(&self.pool, role_id)?;
        Ok(())
    }

    pub fn get_roles_for_user(&self, user_id: uuid::Uuid) -> Result<Vec<Role>, TenetError> {
        let user_roles = DbRole::find_by_user(&self.pool, self.id, user_id)?;

        Ok(user_roles.iter().map(Role::from).collect())
    }
}

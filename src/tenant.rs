use crate::{error::TenetError, application::Application, role::Role, user::User};




#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tenant {
    pub id: uuid::Uuid,
    users: Vec::<User>,
    applications: Vec<Application>,
    roles: Vec<Role>
}

impl Tenant {
    pub fn new() -> Self {
        Tenant { 
            id: uuid::Uuid::new_v4(), 
            users: Vec::new(),
            applications: Vec::new(),
            roles: Vec::new() 
        }
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn add_user(&mut self, user: User) -> Result<uuid::Uuid, TenetError>{
        self.users.push(user);

        Ok(self.users.last().unwrap().id)
    }

    pub fn remove_user(&mut self, user_id: uuid::Uuid) -> Result<User, TenetError> {
        if let Some(index) = self.users.iter().position(|u| u.id == user_id) {
            let user = self.users.remove(index);
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

        Ok(self.applications.last().unwrap().id)
    }

    pub fn remove_application(&mut self, application_id: uuid::Uuid) -> Result<Application, TenetError> {
        if let Some(index) = self.applications.iter().position(|a| a.id == application_id) {
            let application = self.applications.remove(index);
            return Ok(application);
        }
        Err(TenetError::NotFoundError)
    }

    pub fn get_roles(&self) -> &Vec<Role> {
        &self.roles
    }

    pub fn add_role(&mut self, role: Role) -> Result<uuid::Uuid, TenetError> {
        self.roles.push(role);

        Ok(self.roles.last().unwrap().id)
    }

    pub fn remove_role(&mut self, role_id: uuid::Uuid) -> Result<Role, TenetError> {
        if let Some(index) = self.roles.iter().position(|r| r.id == role_id) {
            let role = self.roles.remove(index);
            return Ok(role);
        }
        Err(TenetError::NotFoundError)
    }
}
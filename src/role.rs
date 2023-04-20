



#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum RoleType {
    Administrator,
    User
}


#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Role { 
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub application_id: uuid::Uuid,
    pub role_type: RoleType
}

impl Role {
    pub fn new(user_id: uuid::Uuid, application_id: uuid::Uuid, role_type: RoleType) -> Self {
        Role { 
            id: uuid::Uuid::new_v4(), 
            user_id, 
            application_id,
            role_type }
    }
}
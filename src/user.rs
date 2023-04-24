



#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub encryption_mode: EncryptionModes,
    pub email: String,
    pub email_verified: bool,
    pub full_name: String
}


#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum EncryptionModes {
    Argon2
}


impl User {
    pub fn new(username: String, full_name: String, password: String, encryption_mode: EncryptionModes, email: String, email_verified: bool) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            username,
            password,
            encryption_mode,
            email,
            email_verified,
            full_name,
        }
    }
}




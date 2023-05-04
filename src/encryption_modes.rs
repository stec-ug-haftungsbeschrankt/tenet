use std::{str::FromStr, fmt::Display};

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub enum EncryptionModes {
    Argon2
}


impl FromStr for EncryptionModes {
    type Err = ();

    fn from_str(input: &str) -> Result<EncryptionModes, Self::Err> {
        match input {
            "Argon2"  => Ok(EncryptionModes::Argon2),
            _      => Err(()),
        }
    }
}

impl Display for EncryptionModes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

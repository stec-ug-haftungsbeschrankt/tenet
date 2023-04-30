use std::{str::FromStr, fmt::Display};


#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub enum RoleType {
    Administrator,
    User
}

impl FromStr for RoleType {
    type Err = ();

    fn from_str(input: &str) -> Result<RoleType, Self::Err> {
        match input {
            "Administrator"  => Ok(RoleType::Administrator),
            "User" => Ok(RoleType::User),
            _  => Err(()),
        }
    }
}

impl Display for RoleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

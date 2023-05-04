use std::{str::FromStr, fmt::Display};


#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub enum ApplicationType {
    Shop
}

impl FromStr for ApplicationType {
    type Err = ();

    fn from_str(input: &str) -> Result<ApplicationType, Self::Err> {
        match input {
            "Shop"  => Ok(ApplicationType::Shop),
            _  => Err(()),
        }
    }
}

impl Display for ApplicationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
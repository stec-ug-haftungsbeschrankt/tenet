use std::{str::FromStr, fmt::Display};

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub enum StorageType {
    JsonFile,
    SqliteDatabase,
    PostgreSqlDatabase,
    PostgreSqlSchema,
    PostgreSqlTablePrefix
}


impl FromStr for StorageType {
    type Err = ();

    fn from_str(input: &str) -> Result<StorageType, Self::Err> {
        match input {
            "JsonFile"  => Ok(StorageType::JsonFile),
            "SqliteDatabase" => Ok(StorageType::SqliteDatabase),
            "PostgreSqlDatabase" => Ok(StorageType::PostgreSqlDatabase),
            "PostgreSqlSchema" => Ok(StorageType::PostgreSqlSchema),
            "PostgreSqlTablePrefix" => Ok(StorageType::PostgreSqlTablePrefix),
            _  => Err(()),
        }
    }
}

impl Display for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

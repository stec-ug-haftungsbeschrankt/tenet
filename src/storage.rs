use std::{path::Path, fs::File, io::{Read, Write}};

use crate::{Tenant, error::TenetError};

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Storage {
    JsonFile {
        path: String
    },
    SqliteDatabase {
        path: String
    },
    PostgreSqlDatabase {
        connection_string: String
    },
    PostgreSqlSchema {
        connection_string: String,
        schema_name: String
    }
    ,
    PostgreSqlTablePrefix {
        connection_string: String,
        table_prefix: String
    }
}

impl Storage {
    pub fn read(&self) -> Result<Vec::<Tenant>, TenetError> {
        match self {
            Storage::JsonFile { path } => read_from_json_file(path),
            Storage::SqliteDatabase { path } => todo!(),
            Storage::PostgreSqlDatabase { connection_string } => todo!(),
            Storage::PostgreSqlSchema { connection_string, schema_name } => todo!(),
            Storage::PostgreSqlTablePrefix { connection_string, table_prefix } => todo!(),
        }
    }

    pub fn write(&self, tenants: Vec<Tenant>) -> Result<(), TenetError> {
        match self {
            Storage::JsonFile { path } => write_to_json_file(path, tenants),
            Storage::SqliteDatabase { path } => todo!(),
            Storage::PostgreSqlDatabase { connection_string } => todo!(),
            Storage::PostgreSqlSchema { connection_string, schema_name } => todo!(),
            Storage::PostgreSqlTablePrefix { connection_string, table_prefix } => todo!(),
        }
    }
}

fn read_from_json_file<T>(path: T) -> Result<Vec<Tenant>, TenetError> 
    where T: Into<String> + Copy,
{
    Path::new(&path.into()).try_exists()?;

    let mut file = File::open(path.into())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;    
    
    let tenants: Vec<Tenant> = serde_json::from_str(&contents)?;
    Ok(tenants)
}

fn write_to_json_file<T>(path: T, tenants: Vec<Tenant>) -> Result<(), TenetError> 
    where T: Into<String> + Copy,
{
    Path::new(&path.into()).try_exists()?;

    let data = serde_json::to_string_pretty::<Vec<Tenant>>(&tenants)?;

    let mut file = File::open(path.into())?;
    file.write_all(&data.as_bytes())?;
    Ok(())
}

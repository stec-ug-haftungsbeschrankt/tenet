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
            Storage::SqliteDatabase { path } => read_from_sqlite_database(path),
            Storage::PostgreSqlDatabase { connection_string } => read_from_postgresql_database(connection_string),
            Storage::PostgreSqlSchema { connection_string, schema_name } => read_from_postgresql_database_schema(connection_string, schema_name),
            Storage::PostgreSqlTablePrefix { connection_string, table_prefix } => read_from_postgresql_database_table_prefix(connection_string, table_prefix),
        }
    }

    pub fn write(&self, tenants: &Vec<Tenant>) -> Result<(), TenetError> {
        match self {
            Storage::JsonFile { path } => write_to_json_file(path, tenants),
            Storage::SqliteDatabase { path } => write_to_sqlite_database(path, tenants),
            Storage::PostgreSqlDatabase { connection_string } => write_to_postgresql_database(connection_string, tenants),
            Storage::PostgreSqlSchema { connection_string, schema_name } => write_to_postgresql_database_schema(connection_string, schema_name, tenants),
            Storage::PostgreSqlTablePrefix { connection_string, table_prefix } => write_to_postgresql_database_table_prefix(connection_string, table_prefix, tenants),
        }
    }
}

fn read_from_json_file<T>(path: T) -> Result<Vec<Tenant>, TenetError> 
    where T: Into<String> + Copy,
{
    if !Path::new(&path.into()).exists() {
        File::create(&path.into())?;
    }

    let mut file = File::open(path.into())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;   

    if contents.is_empty() {
        return Ok(Vec::new());
    } 
    
    let tenants: Vec<Tenant> = serde_json::from_str(&contents)?;
    Ok(tenants)
}

fn write_to_json_file<T>(path: T, tenants: &Vec<Tenant>) -> Result<(), TenetError> 
    where T: Into<String> + Copy,
{
    Path::new(&path.into()).try_exists()?;

    let data = serde_json::to_string_pretty::<Vec<Tenant>>(&tenants)?;

    std::fs::write(&path.into(), &data.as_bytes())?;
    //let mut file = File::create(path.into())?;
    //file.write_all(&data.as_bytes())?;
    Ok(())
}


fn read_from_sqlite_database<T>(path: T) -> Result<Vec<Tenant>, TenetError> 
    where T: Into<String> + Copy
{
    todo!()
}

fn write_to_sqlite_database<T>(path: T, tenants: &Vec<Tenant>) -> Result<(), TenetError> 
    where T: Into<String> + Copy
{
    todo!()
}

fn read_from_postgresql_database<T>(connection_string: T) -> Result<Vec<Tenant>, TenetError>
    where T: Into<String> + Copy
{
    todo!()
}

fn write_to_postgresql_database<T>(connection_string: T, tenants: &Vec<Tenant>) -> Result<(), TenetError>
    where T: Into<String> + Copy
{
    todo!()
}

fn read_from_postgresql_database_schema<T>(connection_string: T, schema: T) -> Result<Vec<Tenant>, TenetError>
    where T: Into<String> + Copy
{
    todo!()
}

fn write_to_postgresql_database_schema<T>(connection_string: T, schema: T, tenants: &Vec<Tenant>) -> Result<(), TenetError>
    where T: Into<String> + Copy
{
    todo!()
}

fn read_from_postgresql_database_table_prefix<T>(connection_string: T, table_prefix: T) -> Result<Vec<Tenant>, TenetError>
    where T: Into<String> + Copy
{
    todo!()
}

fn write_to_postgresql_database_table_prefix<T>(connection_string: T, table_prefix: T, tenants: &Vec<Tenant>) -> Result<(), TenetError>
    where T: Into<String> + Copy
{
    todo!()
}
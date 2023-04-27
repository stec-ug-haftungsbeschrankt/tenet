use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use log::info;
use std::env;

use diesel_migrations::EmbeddedMigrations;
use crate::diesel_migrations::MigrationHarness;
use crate::DEFAULT_DATABASE_URL;
use crate::postgresql::service_error::ServiceError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = match env::var("TENET_DATABASE_URL") {
            Ok(url) => url,
            Err(err) => {
                info!("{}", err.to_string());
                info!("Database url not set, using default ConnectionString");
                DEFAULT_DATABASE_URL.to_string()
            }
        };

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}


pub fn initialize_database() {
    info!("Initializing Database");
    lazy_static::initialize(&POOL);
    let mut connection = connection().expect("Failed to get db connection");
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}


pub fn connection() -> Result<DbConnection, ServiceError> {
    POOL.get().map_err(
        |e| ServiceError::new(500, format!("Failed to get database connection: {}", e)))
}

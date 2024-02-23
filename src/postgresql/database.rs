use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::info;
use std::sync::OnceLock;

use diesel_migrations::EmbeddedMigrations;
use crate::diesel_migrations::MigrationHarness;
use crate::CONNECTION_STRING;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static POOL: OnceLock<Pool> = OnceLock::new();

pub fn connection() -> Result<DbConnection, r2d2::Error> {
    let mutex = POOL.get_or_init(|| {       
        info!("Initializing Tenet Database");
        let connection_string = CONNECTION_STRING.get().expect("Unable to get connection string");
        println!("Tadaaa {}", &connection_string);
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        let pool = Pool::new(manager).expect("Failed to create db pool");
        println!("Tadaaa");
        let mut connection = pool.get().expect("Failed to get db connection");
        connection.run_pending_migrations(MIGRATIONS).expect("Unable to run migrations");
        pool
    });
    mutex.get()
}
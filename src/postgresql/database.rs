use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::info;

use diesel_migrations::EmbeddedMigrations;
use crate::diesel_migrations::MigrationHarness;

/// Connection pool type backing a [`crate::Tenet`] instance.
///
/// Each `Tenet` owns its own `Pool`, built from the connection string it was
/// constructed with. This is cheap to clone (it's an `Arc` internally), so
/// multiple `Tenet` instances pointing at different databases can coexist in
/// the same process without interfering with one another.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Builds a new connection pool for the given connection string and runs
/// any pending migrations against it.
pub fn build_pool(connection_string: &str) -> Pool {
    info!("Initializing Tenet Database");
    info!("Tenet connection string: {}", connection_string);

    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    let pool = Pool::new(manager).expect("Failed to create db pool");

    info!("Running pending database migrations...");
    let mut connection = pool.get().expect("Failed to get db connection");
    connection.run_pending_migrations(MIGRATIONS).expect("Unable to run migrations");
    info!("Database migrations completed");

    pool
}

pub fn connection(pool: &Pool) -> Result<DbConnection, r2d2::Error> {
    pool.get()
}

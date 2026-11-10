use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use log::info;

use diesel_migrations::EmbeddedMigrations;
use crate::diesel_migrations::MigrationHarness;
use crate::error::TenetError;

/// Connection pool type backing a [`crate::Tenet`] instance.
///
/// Each `Tenet` owns its own `Pool`, built from the connection string it was
/// constructed with. This is cheap to clone (it's an `Arc` internally), so
/// multiple `Tenet` instances pointing at different databases can coexist in
/// the same process without interfering with one another.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Key of the Postgres advisory lock that serialises migrations across
/// connections, processes and hosts ("TENETMIG" as ASCII bytes).
const MIGRATION_LOCK_KEY: i64 = 0x54454E45544D4947;

/// Strips the userinfo part (`user:password@`) from a connection string so it
/// can be logged safely.
pub fn redact_connection_string(connection_string: &str) -> String {
    match connection_string.split_once("://") {
        Some((scheme, rest)) => {
            let authority_end = rest.find(['/', '?']).unwrap_or(rest.len());
            match rest[..authority_end].rfind('@') {
                Some(at) => format!("{}://***@{}", scheme, &rest[at + 1..]),
                None => connection_string.to_string(),
            }
        }
        None => "<redacted>".to_string(),
    }
}

/// Runs pending migrations while holding a session-level Postgres advisory
/// lock, so concurrent callers (threads, processes, pods) run them one after
/// another. Whoever gets the lock second finds the work already done.
fn run_migrations_locked(connection: &mut PgConnection) -> Result<(), TenetError> {
    diesel::sql_query("SELECT pg_advisory_lock($1)")
        .bind::<diesel::sql_types::BigInt, _>(MIGRATION_LOCK_KEY)
        .execute(connection)?;

    let result = connection
        .run_pending_migrations(MIGRATIONS)
        .map(|_| ())
        .map_err(|e| TenetError::MigrationError(e.to_string()));

    // The lock is also released when the connection closes, so a failed unlock
    // must not mask the migration result.
    let _ = diesel::sql_query("SELECT pg_advisory_unlock($1)")
        .bind::<diesel::sql_types::BigInt, _>(MIGRATION_LOCK_KEY)
        .execute(connection);

    result
}

/// Builds a new connection pool for the given connection string and runs
/// any pending migrations against it.
pub fn build_pool(connection_string: &str) -> Result<Pool, TenetError> {
    info!("Initializing Tenet Database");
    info!("Tenet connection string: {}", redact_connection_string(connection_string));

    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    let pool = Pool::new(manager)?;

    info!("Running pending database migrations...");
    let mut connection = pool.get()?;
    run_migrations_locked(&mut connection)?;
    info!("Database migrations completed");

    drop(connection);
    Ok(pool)
}

pub fn connection(pool: &Pool) -> Result<DbConnection, r2d2::Error> {
    pool.get()
}

#[cfg(test)]
mod tests {
    use super::redact_connection_string;

    #[test]
    fn redacts_userinfo() {
        assert_eq!(
            "postgres://***@localhost:5432/db?sslmode=disable",
            redact_connection_string("postgres://user:secret@localhost:5432/db?sslmode=disable")
        );
    }

    #[test]
    fn redacts_password_containing_at_sign() {
        assert_eq!(
            "postgres://***@host/db",
            redact_connection_string("postgres://user:p@ss@host/db")
        );
    }

    #[test]
    fn leaves_string_without_userinfo() {
        assert_eq!("postgres://localhost/db", redact_connection_string("postgres://localhost/db"));
    }

    #[test]
    fn does_not_treat_at_in_path_or_query_as_userinfo() {
        assert_eq!("postgres://host/db?x=a@b", redact_connection_string("postgres://host/db?x=a@b"));
    }

    #[test]
    fn redacts_unparseable_string_completely() {
        assert_eq!("<redacted>", redact_connection_string("host=localhost password=secret"));
    }
}

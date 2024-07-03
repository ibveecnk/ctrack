//! Database package

use derivative::Derivative;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use eyre::bail;
use std::env;

use diesel::{sqlite::Sqlite, Connection, SqliteConnection};

/// Embedded migrations that might need to be applied
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// Database struct
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Database {
    /// the database connection
    #[derivative(Debug = "ignore")]
    connection: SqliteConnection,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    /// Creates a new database connection
    /// # Panics
    /// - If the `DATABASE_URL` environment variable is not set
    /// - If the migrations fail
    #[must_use]
    pub fn new() -> Self {
        let database_url: String =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        let mut connection = diesel::sqlite::SqliteConnection::establish(database_url.as_str())
            .expect("DATABASE_URL not set");

        run_migrations(&mut connection).expect("We can't resume operation when migrations fail");

        Self { connection }
    }

    /// Get the underlying database connection
    pub fn connection(&mut self) -> &mut SqliteConnection {
        &mut self.connection
    }
}

/// Performs all necessary migrations
fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> eyre::Result<()> {
    let result = connection.run_pending_migrations(MIGRATIONS);

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            bail!(e)
        }
    }
}

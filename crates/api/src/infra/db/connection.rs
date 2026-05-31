use std::str::FromStr;

use sqlx::{
    Executor, Pool, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteQueryResult, SqliteRow},
};
use types::Url;

/// The primary database engine used by the application.
pub type Database = Sqlite;
/// A connection pool tailored for the application's database backend.
pub type DatabasePool = Pool<Database>;
/// A single row resulting from a database query.
pub type DatabaseRow = SqliteRow;

/// A convenience trait alias for anything that can execute queries against our [`Database`].
///
/// This simplifies trait bounds in functions that need to accept either a connection pool,
/// a single connection, or a transaction.
pub trait DatabaseExecutor<'c>: Executor<'c, Database = Database> {}

impl<'c, T: Executor<'c, Database = Database>> DatabaseExecutor<'c> for T {}

/// The result of executing a database query (e.g., number of rows affected).
pub type DatabaseQueryResult = SqliteQueryResult;

/// Establishes a connection pool to the database using the provided URL.
///
/// This function sets up the `SQLite` connection with several performance and integrity
/// configurations out of the box:
///
/// * Creates the database file automatically if it does not exist.
/// * Enables Write-Ahead Logging (`WAL`) mode for better concurrency and performance.
/// * Enforces foreign key constraints to maintain relational integrity.
/// * Triggers a database optimization (PRAGMA optimize) when the pool is closed.
///
/// # Errors
/// Returns a [`sqlx::Error`] if the URL cannot be parsed or if the connection fails to establish.
#[tracing::instrument(level = "trace", err(Debug))]
pub async fn database_connect(database_url: &Url) -> sqlx::Result<DatabasePool> {
    let options = SqliteConnectOptions::from_str(database_url.as_str())?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true)
        .optimize_on_close(true, None);

    SqlitePool::connect_with(options)
        .await
        .inspect(|_| tracing::info!("database connections pool created"))
}

/// Gracefully closes the database connection pool.
#[tracing::instrument(level = "trace", skip_all)]
pub async fn database_close(pool: &DatabasePool) {
    tracing::trace!(pool_size = pool.size(), num_idle = pool.num_idle());
    pool.close().await;
    tracing::info!("database connections pool closed");
}

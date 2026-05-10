use std::str::FromStr;

use sqlx::{
    Pool, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};

pub type Database = Sqlite;
pub type DatabasePool = Pool<Database>;

#[tracing::instrument(level = "trace", err)]
pub async fn database_connect(database_url: &str) -> sqlx::Result<DatabasePool> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true)
        .optimize_on_close(true, None);
    tracing::trace!(?options);

    SqlitePool::connect_with(options)
        .await
        .inspect(|_| tracing::info!("database connections pool created"))
}

#[tracing::instrument(level = "trace", skip_all)]
pub async fn database_close(pool: DatabasePool) {
    tracing::trace!(pool_size = pool.size(), num_idle = pool.num_idle());
    pool.close().await;
    tracing::info!("database connections pool closed");
}

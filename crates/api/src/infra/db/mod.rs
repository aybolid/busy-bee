use std::{ops::Deref, str::FromStr};

use sqlx::{
    Acquire, Executor, Pool, Sqlite, SqlitePool,
    migrate::Migrate,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteQueryResult, SqliteRow},
};

pub type Database = Sqlite;
pub type DatabasePool = Pool<Database>;
pub type DatabaseRow = SqliteRow;

pub trait DatabaseExecutor<'c>: Executor<'c, Database = Database> {}

impl<'c, T: Executor<'c, Database = Database>> DatabaseExecutor<'c> for T {}

pub type DatabaseQueryResult = SqliteQueryResult;

#[tracing::instrument(level = "trace", err(Debug))]
pub async fn database_connect(database_url: &str) -> sqlx::Result<DatabasePool> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true)
        .optimize_on_close(true, None);

    SqlitePool::connect_with(options)
        .await
        .inspect(|_| tracing::info!("database connections pool created"))
}

#[tracing::instrument(level = "trace", skip_all)]
pub async fn database_close(pool: &DatabasePool) {
    tracing::trace!(pool_size = pool.size(), num_idle = pool.num_idle());
    pool.close().await;
    tracing::info!("database connections pool closed");
}

#[tracing::instrument(level = "trace", skip_all, err(Debug))]
pub async fn database_migrate<'a, A>(migrator: A) -> Result<(), sqlx::migrate::MigrateError>
where
    A: Acquire<'a>,
    <A::Connection as Deref>::Target: Migrate,
{
    sqlx::migrate!("src/infra/db/migrations")
        .run(migrator)
        .await
        .inspect(|()| tracing::info!("database migrations applied"))
}

use std::ops::Deref;

use sqlx::{Acquire, migrate::Migrate};

/// Executes pending database migrations against the target database.
///
/// This function uses the `sqlx::migrate!` macro to embed the SQL migration scripts
/// located in the `src/infra/db/migrations` directory directly into the application
/// binary at compile time. It then applies any migrations that have not yet been
/// run against the database.
///
/// The `migrator` argument leverages generics to accept any type that can yield a
/// database connection capable of performing migrations. This means you can pass a
/// `DatabasePool`, a single database connection, or an active transaction.
///
/// # Errors
/// Returns a [`sqlx::migrate::MigrateError`] if a migration script contains invalid SQL,
/// if there is a database connection issue, or if the migration history is inconsistent.
pub async fn database_migrate<'a, A>(migrator: A) -> Result<(), sqlx::migrate::MigrateError>
where
    A: Acquire<'a>,
    <A::Connection as Deref>::Target: Migrate,
{
    sqlx::migrate!("src/infra/db/migrations")
        .run(migrator)
        .await
}

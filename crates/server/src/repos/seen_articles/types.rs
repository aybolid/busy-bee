use std::num::NonZeroU32;

use types::Url;

/// A unique, non-zero identifier for a [`SeenArticle`].
///
/// This type wraps a `NonZeroU32` and maps transparently to the database's
/// internal integer primary key via `sqlx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
#[sqlx(transparent)]
#[allow(dead_code)]
pub struct SeenArticleId(NonZeroU32);

/// Represents a lightweight record of a previously processed article URL.
///
/// This struct maps directly to the `seen_articles` database table, which acts
/// as a fast, persistent ledger to prevent redundant processing of the same
/// items during RSS fetches.
#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct SeenArticle {
    pub id: SeenArticleId,
    pub url: Url,
}

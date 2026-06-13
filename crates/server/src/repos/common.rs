use std::num::{NonZeroU8, NonZeroU32};

use types::{LengthBounded, TrimmedString};

/// Defines pagination parameters for querying collections of items.
#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct Pagination {
    pub page_index: usize,
    pub limit: NonZeroU8,
}

impl Pagination {
    /// Returns the current page index.
    pub fn page_index(&self) -> usize {
        self.page_index
    }

    /// Returns the maximum number of items to retrieve per page.
    pub fn limit(&self) -> NonZeroU8 {
        self.limit
    }

    /// Calculates and returns a `(limit, offset)` tuple,
    /// typically used for SQL `LIMIT` and `OFFSET` clauses.
    #[allow(clippy::cast_possible_wrap)]
    pub fn as_limit_and_offset(&self) -> (NonZeroU8, i64) {
        (
            self.limit,
            (self.page_index * usize::from(self.limit.get())) as i64,
        )
    }
}

/// Number used for data versioning in the database.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct VersionNumber(pub NonZeroU32);

impl std::ops::Deref for VersionNumber {
    type Target = NonZeroU32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct SearchString(pub LengthBounded<2, { u8::MAX as usize }, TrimmedString>);

impl std::ops::Deref for SearchString {
    type Target = LengthBounded<2, { u8::MAX as usize }, TrimmedString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for SearchString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl SearchString {
    /// Converts the search string into a safely escaped pattern suitable for use
    /// in SQL `LIKE` queries.
    ///
    /// This method prepares the string for a partial match (e.g., `LIKE '%keyword%'`)
    /// while protecting against wildcard injection. It performs two main operations:
    ///
    /// 1. **Escapes special characters:** Prepend backslashes to `\`, `%`, and `_`
    ///    so the database treats them as literal characters rather than wildcards.
    /// 2. **Adds wildcards:** Wraps the entire escaped string in `%` to match the
    ///    pattern anywhere within a database column.
    pub fn to_like_pattern(&self) -> String {
        let escaped = self
            .to_string()
            .replace('\\', r"\\")
            .replace('%', r"\%")
            .replace('_', r"\_");

        format!("%{escaped}%")
    }
}

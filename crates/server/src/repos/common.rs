use std::num::NonZeroU8;

/// Defines pagination parameters for querying collections of items.
#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct Pagination {
    page_index: usize,
    limit: NonZeroU8,
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

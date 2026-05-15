use crate::internal::repos::types::non_empty::EmptyCheck;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct TrimmedString(String);

impl TrimmedString {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(string: impl AsRef<str>) -> Self {
        Self(string.as_ref().trim().to_owned())
    }
}

impl std::ops::Deref for TrimmedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<String> for TrimmedString {
    fn as_ref(&self) -> &String {
        self
    }
}

impl EmptyCheck for TrimmedString {
    fn is_empty(&self) -> bool {
        EmptyCheck::is_empty(&self.0)
    }
}

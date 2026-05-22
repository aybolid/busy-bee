use crate::internal::repos::types::length::LengthCheck;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct TrimmedString(String);

impl TrimmedString {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(string: impl AsRef<str>) -> Self {
        Self(string.as_ref().trim().to_owned())
    }
}

impl<'de> serde::Deserialize<'de> for TrimmedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::new(s))
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

impl LengthCheck for TrimmedString {
    fn len(&self) -> usize {
        String::len(self)
    }
}

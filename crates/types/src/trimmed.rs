/// A wrapper that guarantees the inner string is always trimmed
/// of leading and trailing whitespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct TrimmedString(String);

impl TrimmedString {
    /// Creates a new `TrimmedString`, allocating a new trimmed inner `String`.
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(s.trim().to_owned())
    }

    /// Returns a shared reference to the underlying `String`.
    #[must_use]
    pub fn inner(&self) -> &String {
        self
    }

    /// Consumes the wrapper, returning the underlying `String`.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::str::FromStr for TrimmedString {
    type Err = std::convert::Infallible; // Trimming never "fails"

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TrimmedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from(String::deserialize(deserializer)?))
    }
}

impl From<String> for TrimmedString {
    fn from(value: String) -> Self {
        if value.trim().len() == value.len() {
            // Re-use the allocation if it is already trimmed
            Self(value)
        } else {
            Self::new(&value)
        }
    }
}

impl From<&str> for TrimmedString {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl std::ops::Deref for TrimmedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for TrimmedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TrimmedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        String::fmt(self, f)
    }
}

impl std::borrow::Borrow<str> for TrimmedString {
    fn borrow(&self) -> &str {
        self
    }
}

#[cfg(feature = "sqlx")]
impl<DB: sqlx::Database> sqlx::Type<DB> for TrimmedString
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as sqlx::Database>::TypeInfo) -> bool {
        <String as sqlx::Type<DB>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx")]
impl<'q, DB: sqlx::Database> sqlx::Encode<'q, DB> for TrimmedString
where
    String: sqlx::Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<'q, DB>>::encode_by_ref(self, buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for TrimmedString
where
    String: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<'r, DB>>::decode(value)?;
        Ok(Self::from(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_inner_value() {
        let s = TrimmedString::new("  hello \t");
        assert_eq!(s.inner(), "hello");
    }

    #[test]
    fn from_string_reuses_allocation_if_already_trimmed() {
        let string = String::from("hello");
        let p = string.as_ptr();
        let trimmed = TrimmedString::from(string);
        assert_eq!(p, trimmed.as_ptr());
    }
}

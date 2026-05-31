/// A validated, strongly-typed URL.
///
/// This is a newtype wrapper around [`url::Url`]. It provides transparent integration
/// with serialization (`serde`) and database operations (`sqlx`) while ensuring
/// that the underlying string is always a correctly formatted URL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Url(url::Url);

/// Represents an error encountered while attempting to parse a URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error(transparent)]
pub struct UrlError(#[from] url::ParseError);

impl Url {
    /// Attempts to parse a string slice into a new [`Url`].
    ///
    /// Returns [`None`] if the provided string is not a valid URL.
    #[must_use]
    pub fn new(s: &str) -> Option<Self> {
        Self::try_new(s).ok()
    }

    /// Attempts to parse a string slice into a new [`Url`], returning a detailed error on failure.
    ///
    /// # Errors
    ///
    /// Returns a [`UrlError`] if the input string violates URL formatting rules.
    pub fn try_new(s: &str) -> Result<Self, UrlError> {
        let url = url::Url::parse(s)?;
        Ok(Self(url))
    }
}

impl std::ops::Deref for Url {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "sqlx")]
impl<DB: sqlx::Database> sqlx::Type<DB> for Url
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
impl<'q, DB: sqlx::Database> sqlx::Encode<'q, DB> for Url
where
    String: sqlx::Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<'q, DB>>::encode_by_ref(&self.to_string(), buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for Url
where
    String: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<'r, DB>>::decode(value)?;
        let url = Self::try_new(&s)?;
        Ok(url)
    }
}

impl std::str::FromStr for Url {
    type Err = UrlError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::try_new(input)
    }
}

impl<'a> TryFrom<&'a str> for Url {
    type Error = UrlError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::try_new(s)
    }
}

impl std::fmt::Display for Url {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.as_str(), formatter)
    }
}

impl From<Url> for String {
    fn from(value: Url) -> Self {
        value.to_string()
    }
}

impl AsRef<str> for Url {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

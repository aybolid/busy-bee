pub trait LengthCheck {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct MaxLength<const MAX: usize, T: LengthCheck>(T);

#[derive(Debug, thiserror::Error)]
#[error("expected value length to be <= {0}")]
pub struct MaxLengthError(usize);

impl<const MAX: usize, T: LengthCheck> MaxLength<MAX, T> {
    pub fn new(value: T) -> Option<Self> {
        (value.len() <= MAX).then_some(Self(value))
    }

    pub fn try_new(value: T) -> Result<Self, MaxLengthError> {
        Self::new(value).ok_or(MaxLengthError(MAX))
    }
}

impl<'de, const MAX: usize, T: LengthCheck> serde::Deserialize<'de> for MaxLength<MAX, T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

impl<const MAX: usize, T: LengthCheck> std::ops::Deref for MaxLength<MAX, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX: usize, T: LengthCheck> AsRef<T> for MaxLength<MAX, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct NonEmpty<T: LengthCheck>(T);

#[derive(Debug, thiserror::Error)]
#[error("value is empty")]
pub struct EmptyValueError;

impl<T: LengthCheck> NonEmpty<T> {
    pub fn new(value: T) -> Option<Self> {
        (!value.is_empty()).then_some(Self(value))
    }

    pub fn try_new(value: T) -> Result<Self, EmptyValueError> {
        Self::new(value).ok_or(EmptyValueError)
    }
}

impl<T: LengthCheck> std::ops::Deref for NonEmpty<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: LengthCheck> AsRef<T> for NonEmpty<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: LengthCheck> LengthCheck for NonEmpty<T> {
    fn len(&self) -> usize {
        LengthCheck::len(&self.0)
    }
}

impl<'de, T: LengthCheck> serde::Deserialize<'de> for NonEmpty<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

impl LengthCheck for String {
    fn len(&self) -> usize {
        self.chars().count()
    }
}

pub trait EmptyCheck {
    fn is_empty(&self) -> bool;
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct NonEmpty<T: EmptyCheck>(T);

#[derive(Debug, thiserror::Error)]
#[error("value is empty")]
pub struct EmptyValueError;

impl<T: EmptyCheck> NonEmpty<T> {
    pub fn new(value: T) -> Option<Self> {
        (!value.is_empty()).then_some(Self(value))
    }

    pub fn try_new(value: T) -> Result<Self, EmptyValueError> {
        Self::new(value).ok_or(EmptyValueError)
    }
}

impl<T: EmptyCheck> std::ops::Deref for NonEmpty<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: EmptyCheck> AsRef<T> for NonEmpty<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl EmptyCheck for String {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

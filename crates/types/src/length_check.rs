use crate::TrimmedString;

/// A trait for types that have a definable and measurable length.
///
/// This trait abstracts the concept of "length" so that types like [`String`],
/// [`Vec`], and custom wrappers can be uniformly validated by [`LengthBounded`].
pub trait LengthCheck {
    /// Returns the length of the underlying type.
    fn len(&self) -> usize;

    /// Returns `true` if the length is exactly `0`.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl LengthCheck for String {
    /// Returns the number of characters in the string.
    ///
    /// **Performance Note:** This is an `O(N)` operation because it iterates
    /// over the string's UTF-8 characters, not its bytes.
    fn len(&self) -> usize {
        self.chars().count()
    }

    /// Overridden for O(1) performance. A string has 0 characters
    /// if and only if it has 0 bytes.
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl LengthCheck for TrimmedString {
    /// Returns the number of characters in the string.
    ///
    /// **Performance Note:** This is an `O(N)` operation because it iterates
    /// over the string's UTF-8 characters, not its bytes.
    fn len(&self) -> usize {
        LengthCheck::len(self.inner())
    }

    /// Overridden for O(1) performance. A string has 0 characters
    /// if and only if it has 0 bytes.
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<T> LengthCheck for Vec<T> {
    /// Returns the number of elements in the vector.
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthCheck> LengthCheck
    for LengthBounded<MIN, MAX, T>
{
    fn len(&self) -> usize {
        LengthCheck::len(&self.0)
    }
}

/// A wrapper that guarantees the inner type's length falls within
/// an inclusive `[MIN, MAX]` range.
///
/// Because this struct is `#[repr(transparent)]`, it has the exact same memory
/// layout as the inner type `T`, making the validation entirely zero-cost at
/// runtime after initialization.
///
/// # Examples
///
/// ```
/// # use types::LengthBounded;
/// // Requires exactly 5 characters
/// type ExactFive = LengthBounded<5, 5, String>;
///
/// assert!(ExactFive::try_new(String::from("hello")).is_ok());
/// assert!(ExactFive::try_new(String::from("hi")).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct LengthBounded<const MIN: usize, const MAX: usize, T: LengthCheck>(T);

/// The error type returned when attempting to create a [`LengthBounded`]
/// value that violates its length constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub struct LengthBoundError {
    pub min: usize,
    pub max: usize,
    pub actual: usize,
}

impl std::fmt::Display for LengthBoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = self.min;
        let max = self.max;
        let actual = self.actual;

        match (min == usize::MIN, max == usize::MAX) {
            (true, true) => write!(f, "length {actual} is invalid"), // Unlikely to ever trigger
            (true, false) => write!(f, "length {actual} is too long (maximum is {max})"),
            (false, true) => write!(f, "length {actual} is too short (minimum is {min})"),
            (false, false) => {
                if min == max {
                    write!(f, "length {actual} is invalid (must be exactly {min})")
                } else {
                    write!(
                        f,
                        "length {actual} is out of bounds (must be between {min} and {max})",
                    )
                }
            }
        }
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthCheck> LengthBounded<MIN, MAX, T> {
    /// Creates a new `LengthBounded` value, returning `None` if the validation fails.
    ///
    /// If you need to know exactly *why* the validation failed, use [`try_new`] instead.
    ///
    /// [`try_new`]: LengthBounded::try_new
    pub fn new(value: T) -> Option<Self> {
        Self::try_new(value).ok()
    }

    /// Attempts to create a new `LengthBounded` value.
    ///
    /// # Errors
    ///
    /// Returns a [`LengthBoundError`] containing the actual length if the length
    /// of the provided `value` falls outside the inclusive range `[MIN, MAX]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use types::LengthBounded;
    ///
    /// let valid = LengthBounded::<1, 10, String>::try_new(String::from("Valid"));
    /// assert!(valid.is_ok());
    ///
    /// let invalid = LengthBounded::<1, 3, String>::try_new(String::from("Too long"));
    /// assert!(invalid.is_err());
    /// ```
    pub fn try_new(value: T) -> Result<Self, LengthBoundError> {
        let length = value.len();
        if (MIN..=MAX).contains(&length) {
            Ok(Self(value))
        } else {
            Err(LengthBoundError {
                min: MIN,
                max: MAX,
                actual: length,
            })
        }
    }

    /// Returns a shared reference to the inner value.
    pub fn inner(&self) -> &T {
        self
    }

    /// Consumes the wrapper, returning the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[cfg(feature = "serde")]
impl<'de, const MIN: usize, const MAX: usize, T: LengthCheck + serde::Deserialize<'de>>
    serde::Deserialize<'de> for LengthBounded<MIN, MAX, T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LengthBoundedParseError<InnerE: std::error::Error> {
    #[error(transparent)]
    LengthBound(#[from] LengthBoundError),
    #[error(transparent)]
    Inner(InnerE),
}

impl<const MIN: usize, const MAX: usize, T: LengthCheck + std::str::FromStr> std::str::FromStr
    for LengthBounded<MIN, MAX, T>
where
    <T as std::str::FromStr>::Err: std::error::Error,
{
    type Err = LengthBoundedParseError<<T as std::str::FromStr>::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse().map_err(LengthBoundedParseError::Inner)?;
        Ok(Self::try_new(value)?)
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthCheck> std::ops::Deref
    for LengthBounded<MIN, MAX, T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthCheck> AsRef<T> for LengthBounded<MIN, MAX, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthCheck + std::fmt::Display> std::fmt::Display
    for LengthBounded<MIN, MAX, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner().fmt(f)
    }
}

impl<const MIN: usize, const MAX: usize, T: LengthCheck> std::borrow::Borrow<T>
    for LengthBounded<MIN, MAX, T>
{
    fn borrow(&self) -> &T {
        self
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<str> for LengthBounded<MIN, MAX, String> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const MIN: usize, const MAX: usize> AsRef<str> for LengthBounded<MIN, MAX, TrimmedString> {
    fn as_ref(&self) -> &str {
        self
    }
}

/// A type alias for a value that is guaranteed to have a length of at least `{ usize::MIN + 1 }`.
#[allow(clippy::identity_op)]
pub type NonEmpty<T> = LengthBounded<{ usize::MIN + 1 }, { usize::MAX }, T>;

/// A type alias for a value that is guaranteed to have a maximum length limit.
pub type MaxLength<const MAX: usize, T> = LengthBounded<{ usize::MIN }, MAX, T>;

/// A type alias that combines [`NonEmpty`] and [`MaxLength`].
#[allow(clippy::identity_op)]
pub type NonEmptyMaxLength<const MAX: usize, T> = LengthBounded<{ usize::MIN + 1 }, MAX, T>;

/// A type alias for a value that is guaranteed to have a minimum length limit.
pub type MinLength<const MIN: usize, T> = LengthBounded<MIN, { usize::MAX }, T>;

/// A type alias for a value that is guaranteed to have an exact length.
pub type ExactLength<const EXACT: usize, T> = LengthBounded<EXACT, EXACT, T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_check_string() {
        let ascii = String::from("hello");
        assert_eq!(LengthCheck::len(&ascii), 5);
        assert!(!LengthCheck::is_empty(&ascii));

        // UTF-8 string: 1 character, but 4 bytes under the hood.
        // This ensures our String implementation correctly uses `.chars().count()`
        let emoji = String::from("🚀");
        assert_eq!(emoji.len(), 4); // Standard String::len() is byte length
        assert_eq!(LengthCheck::len(&emoji), 1); // Our LengthCheck::len() is char count
    }

    #[test]
    fn test_length_check_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(LengthCheck::len(&v), 3);

        let empty: Vec<i32> = vec![];
        assert!(LengthCheck::is_empty(&empty));
    }

    #[test]
    fn test_bounded_exact_match() {
        // Must be exactly 3 elements
        type ExactThree<T> = LengthBounded<3, 3, T>;

        assert!(ExactThree::new(vec![1, 2, 3]).is_some());
        assert!(ExactThree::new(vec![1, 2]).is_none());
        assert!(ExactThree::new(vec![1, 2, 3, 4]).is_none());
    }

    #[test]
    fn test_error_display_formatting() {
        // Test MaxLength fallback (MIN == 0)
        let max_err = LengthBoundError {
            min: usize::MIN,
            max: 5,
            actual: 8,
        };
        assert_eq!(max_err.to_string(), "length 8 is too long (maximum is 5)");

        // Test MinLength fallback (MAX == usize::MAX)
        let min_err = LengthBoundError {
            min: 4,
            max: usize::MAX,
            actual: 2,
        };
        assert_eq!(min_err.to_string(), "length 2 is too short (minimum is 4)");

        // Test Exact Match Error (MIN == MAX)
        let exact_err = LengthBoundError {
            min: 5,
            max: 5,
            actual: 3,
        };
        assert_eq!(
            exact_err.to_string(),
            "length 3 is invalid (must be exactly 5)"
        );

        // Test Range Error (MIN != MAX)
        let range_err = LengthBoundError {
            min: 3,
            max: 8,
            actual: 10,
        };
        assert_eq!(
            range_err.to_string(),
            "length 10 is out of bounds (must be between 3 and 8)"
        );
    }

    #[test]
    fn test_type_aliases() {
        // NonEmpty
        assert!(NonEmpty::try_new(String::from("A")).is_ok());
        assert!(NonEmpty::try_new(String::new()).is_err());

        // MaxLength
        assert!(MaxLength::<3, _>::try_new(vec![1, 2, 3]).is_ok());
        assert!(MaxLength::<3, _>::try_new(vec![1, 2, 3, 4]).is_err());

        // MinLength
        assert!(MinLength::<2, _>::try_new(vec![1, 2]).is_ok());
        assert!(MinLength::<2, _>::try_new(vec![1]).is_err());

        // ExactLength
        assert!(ExactLength::<2, _>::try_new(vec![1, 2]).is_ok());
        assert!(ExactLength::<2, _>::try_new(vec![1]).is_err());
        assert!(ExactLength::<2, _>::try_new(vec![1, 2, 3]).is_err());
    }

    #[test]
    fn test_trait_delegations() {
        let original = String::from("hello world");
        let bounded = NonEmpty::try_new(original.clone()).unwrap();

        // Test Deref (derefs to String, which then derefs to str)
        assert_eq!(bounded.as_str(), "hello world");
        assert_eq!(bounded.capacity(), original.capacity());

        // Test AsRef
        let as_ref: &String = bounded.as_ref();
        assert_eq!(as_ref, &original);

        // Test Borrow
        let borrowed: &String = std::borrow::Borrow::borrow(&bounded);
        assert_eq!(borrowed, &original);

        // Test Display
        assert_eq!(format!("{bounded}"), "hello world");
    }

    #[test]
    fn test_into_inner() {
        let v = vec![1, 2, 3];
        let bounded = NonEmpty::try_new(v.clone()).unwrap();

        let extracted = bounded.into_inner();
        assert_eq!(extracted, v);
    }
}

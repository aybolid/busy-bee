#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::pedantic
)]

mod length_check;
mod trimmed;

pub use length_check::{
    ExactLength, LengthBoundError, LengthBounded, LengthBoundedParseError, LengthCheck, MaxLength,
    MinLength, NonEmpty, NonEmptyMaxLength,
};
pub use trimmed::TrimmedString;

#[macro_export]
macro_rules! nonempty_trimmed_string {
    ($val:expr) => {{
        use $crate::{NonEmpty, TrimmedString};

        const fn is_valid(s: &str) -> bool {
            let bytes = s.as_bytes();
            let mut i = 0;

            while i < bytes.len() {
                let b = bytes[i];
                // Check for common ASCII whitespace
                if b != b' ' && b != b'\t' && b != b'\n' && b != b'\r' {
                    return true; // Found a non-whitespace character
                }
                i += 1;
            }

            false // Empty or only whitespace
        }

        const VAL: &str = $val;

        const _: () = assert!(
            is_valid(VAL),
            "String slice cannot be empty or contain only whitespace."
        );

        NonEmpty::new(TrimmedString::new(VAL)).unwrap()
    }};
}

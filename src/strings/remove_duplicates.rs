use alloc::{string::String, vec::Vec};

/// Remove duplicates from sentence
///
/// # Arguments
///
/// * `text` - The text to be processed
///
/// # Returns
///
/// Returns the text without duplicates
#[must_use]
pub fn remove_duplicates(text: &str) -> String {
    text.split_whitespace()
        .fold(Vec::new(), |mut init, t| {
            if init.contains(&t) {
                init
            } else {
                init.push(t);
                init
            }
        })
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Rust is great and Java is also great", "Rust is great and Java also")]
    fn check_removed_duplicates(text: &str, expected: &str) {
        let actual = remove_duplicates(text);
        assert_eq!(expected, actual);
    }
}

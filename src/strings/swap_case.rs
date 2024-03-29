use alloc::string::String;

/// This function will convert all lowercase letters to uppercase letters and vice versa
///
/// # Arguments
///
/// * `text` - The text to be processed
///
/// # Returns
///
/// Returns the text with all lowercase letters converted to uppercase letters and vice versa
#[must_use]
pub fn swap_case(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .fold(String::new(), |mut s, ch| -> String {
            s.push(ch);
            s
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Algorithm", "aLGORITHM")]
    fn check_case_swap(text: &str, expected: &str) {
        let actual = swap_case(text);
        assert_eq!(expected, actual);
    }
}

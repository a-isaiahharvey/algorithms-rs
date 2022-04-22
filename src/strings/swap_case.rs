/// This function will convert all lowercase letters to uppercase letters and vice versa
pub fn swap_case(text: String) -> String {
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

    #[test_case(String::from("Algorithm"), String::from("aLGORITHM"))]
    fn check_case_swap(text: String, expected: String) {
        let actual = swap_case(text);
        assert_eq!(expected, actual);
    }
}

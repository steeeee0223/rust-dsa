pub fn repeated_substring_pattern(s: String) -> bool {
    let n = s.len();

    (1..n / 2 + 1)
        .filter(|i| n % i == 0)
        .any(|i| s[..i].repeat(n / i) == s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abab", true; "case 1")]
    #[test_case("aba", false; "case 2")]
    #[test_case("abcabcabc", true; "case 3")]
    fn test_repeated_substring_pattern(s: &str, expected: bool) {
        let result = repeated_substring_pattern(String::from(s));
        assert_eq!(expected, result);
    }
}

pub fn to_lower_case(s: String) -> String {
    s.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Hello", "hello"; "case 1")]
    #[test_case("here", "here"; "case 2")]
    #[test_case("LOVELY", "lovely"; "case 3")]
    fn test_to_lower_case(x: &str, expected: &str) {
        let result = to_lower_case(String::from(x));
        assert_eq!(String::from(expected), result);
    }
}

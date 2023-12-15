pub fn find_the_difference(s: String, t: String) -> char {
    s.as_bytes() // convert to 8-bit bytes
        .iter() // iterate
        .chain(t.as_bytes()) // chained with another iterator (8-bit bytes of t)
        .fold(0, |acc, &x| acc ^ x) as char // reduce
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abcd", "abcde", 'e'; "case 1")]
    #[test_case("", "y", 'y'; "case 2")]
    #[test_case("love", "ovale", 'a'; "case 3")]
    fn test_find_the_difference(s: &str, t: &str, expected: char) {
        let result = find_the_difference(String::from(s), String::from(t));
        assert_eq!(expected, result);
    }
}

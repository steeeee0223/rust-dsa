pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map_or(-1, |x| x as i32)
}

pub fn str_str2(haystack: String, needle: String) -> i32 {
    let size = needle.len();
    let needle = needle.as_bytes();
    if size == 0 {
        return 0;
    }

    haystack
        .as_bytes()
        .windows(size)
        .position(|slice| slice == needle)
        .map_or(-1, |x| x as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("sadbutsad", "sad", 0; "case 1")]
    #[test_case("leetcode", "leeto", -1; "case 2")]

    fn test_str_str(haystack: &str, needle: &str, expected: i32) {
        let result = str_str(String::from(haystack), String::from(needle));
        assert_eq!(expected, result);
    }
}

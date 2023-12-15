use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.values().all(|i| *i == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case( "anagram",  "nagaram", true;"case 1")]
    #[test_case( "rat",  "car", false;"case 2")]
    fn test_is_anagram(s: &str, t: &str, expected: bool) {
        let result = is_anagram(String::from(s), String::from(t));
        assert_eq!(result, expected)
    }
}

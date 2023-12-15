use std::cmp;

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();
    let [m, n] = [word1.len(), word2.len()];

    for i in 0..cmp::max(m, n) {
        if i < m {
            result.push_str(&word1[i..i + 1]);
        }
        if i < n {
            result.push_str(&word2[i..i + 1]);
        }
    }

    result
}

/**
 * provided by `zakharovvi`
 */
pub fn merge_alternately2(word1: String, word2: String) -> String {
    word1
        .chars()
        .zip(word2.chars())
        .flat_map(|(c1, c2)| [c1, c2])
        .chain(word1.chars().skip(word2.len()))
        .chain(word2.chars().skip(word1.len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abc", "pqr", "apbqcr"; "case 1")]
    #[test_case("ab", "pqrs", "apbqrs"; "case 2")]
    #[test_case("abcd", "pq", "apbqcd"; "case 3")]
    fn test_merge_alternately(x: &str, y: &str, expected: &str) {
        let result = merge_alternately(String::from(x), String::from(y));
        assert_eq!(String::from(expected), result);
    }
}

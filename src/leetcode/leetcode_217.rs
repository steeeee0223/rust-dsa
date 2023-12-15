use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut s = HashSet::new();

    for num in nums {
        if s.contains(&num) {
            return true;
        }
        s.insert(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[1, 2, 3, 1], true; "case 1")]
    #[test_case(&[1, 2, 3, 4], false; "case 2")]
    #[test_case(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true; "case 3")]
    fn test_contains_duplicate(nums: &[i32], expected: bool) {
        let result = contains_duplicate(nums.to_vec());
        assert_eq!(result, expected);
    }
}

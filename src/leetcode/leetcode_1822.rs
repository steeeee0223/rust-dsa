pub fn array_sign(nums: Vec<i32>) -> i32 {
    nums.iter().fold(1, |acc, &num| acc * num.signum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[-1, -2, -3, -4, 3, 2, 1], 1; "case 1")]
    #[test_case(&[1, 5, 0, 2, -3], 0; "case 2")]
    #[test_case(&[-1, 1, -1, 1, -1], -1; "case 3")]
    fn test_array_sign(nums: &[i32], expected: i32) {
        let result = array_sign(nums.to_vec());
        assert_eq!(result, expected);
    }
}

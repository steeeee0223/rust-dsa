pub fn is_monotonic(nums: Vec<i32>) -> bool {
    if nums.len() <= 2 {
        return true;
    }

    let acc = nums.windows(2).all(|slice| slice[1] >= slice[0]);
    let dec = nums.windows(2).all(|slice| slice[1] <= slice[0]);
    acc || dec
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[1,2,3,3], true; "case 1")]
    #[test_case(&[6,5,4,4], true; "case 2")]
    #[test_case(&[1,3,2], false; "case 3")]
    #[test_case(&[2,2,2,1,4,5], false; "case 4")]
    fn test_is_monotonic(nums: &[i32], expected: bool) {
        let result = is_monotonic(nums.to_vec());
        assert_eq!(result, expected);
    }
}

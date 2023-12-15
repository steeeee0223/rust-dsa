pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != 0 {
            nums.swap(i, j);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[0, 1, 0, 3, 12], &[1, 3, 12, 0, 0]; "case 1")]
    #[test_case(&[0], &[0]; "case 2")]
    fn test_move_zeroes(nums: &[i32], expected: &[i32]) {
        let mut nums = Vec::from(nums);
        move_zeroes(&mut nums);
        assert_eq!(Vec::from(expected), nums);
    }
}

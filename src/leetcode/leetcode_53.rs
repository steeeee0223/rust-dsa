use std::cmp::max;
use std::i32::MIN;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums.iter().fold(MIN, |acc, &num| max(acc, num));
    let mut curr = 0;

    for i in 0..nums.len() {
        curr += nums[i];
        if curr > max_sum {
            max_sum = curr
        }
        if curr < 0 {
            curr = 0
        }
    }

    max_sum
}

/**
 * Kadane's Algorithm
 */
pub fn max_sub_array2(nums: Vec<i32>) -> i32 {
    let result = nums.iter().fold((0, MIN), |(curr, max_sum), &num| {
        (max(0, curr + num), max(max_sum, curr + num))
    });
    result.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[-2, 1, -3, 4, -1, 2, 1, -5, 4],  6; "case 1")]
    #[test_case(&[1], 1; "case 2")]
    #[test_case(&[5, 4, -1, 7, 8], 23; "case 3")]
    #[test_case(&[-1], -1; "case 4")]
    fn test_max_sub_array(nums: &[i32], expected: i32) {
        let result = max_sub_array2(nums.to_vec());
        assert_eq!(result, expected);
    }
}

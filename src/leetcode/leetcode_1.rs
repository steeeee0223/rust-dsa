use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let diff = target - num;
        match seen.get(&diff) {
            Some(&j) => return vec![i as i32, j as i32],
            None => {
                seen.insert(num, i);
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[2,7,11,15], 9, [0,1]; "case 1")]
    #[test_case(&[3,2,4], 6, [1,2]; "case 2")]
    #[test_case(&[3,3], 6, [0,1]; "case 3")]

    fn test_two_sum(nums: &[i32], target: i32, expected: [i32; 2]) {
        let mut result = two_sum(nums.to_vec(), target);
        result.sort();
        assert_eq!(result, expected.to_vec());
    }
}

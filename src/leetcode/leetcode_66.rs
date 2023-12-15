pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;

    for i in (0..digits.len()).rev() {
        if digits[i] == 9 {
            digits[i] = 0;
        } else {
            digits[i] += 1;
            return digits;
        }
    }
    digits.insert(0, 1);
    digits
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[1,2,3],&[1,2,4]; "case 1")]
    #[test_case(&[4,3,2,1],&[4,3,2,2]; "case 2")]
    #[test_case(&[9],&[1,0]; "case 3")]
    fn test_plus_one(digits: &[i32], expected: &[i32]) {
        let result = plus_one(digits.to_vec());
        assert_eq!(result, expected.to_vec());
    }
}

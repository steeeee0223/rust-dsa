pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    if arr.len() <= 2 {
        return true;
    }

    let mut arr = arr;
    arr.sort();

    let diff: i32 = arr[1] - arr[0];
    arr.iter()
        .enumerate()
        .skip(1)
        .all(|(i, val)| val - arr[i - 1] == diff)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[3, 5, 1], true; "case 1")]
    #[test_case(&[1, 2, 4], false; "case 2")]
    fn test_can_make_arithmetic_progression(arr: &[i32], expected: bool) {
        let result = can_make_arithmetic_progression(arr.to_vec());
        assert_eq!(result, expected);
    }
}

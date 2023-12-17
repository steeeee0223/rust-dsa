trait FromRoman {
    fn to_roman_int(&self) -> i32;
}

impl FromRoman for char {
    fn to_roman_int(&self) -> i32 {
        match self {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Not a roman numeral"),
        }
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let res = s.chars().rev().fold((0, 0), |(acc, prev), ch| {
        let n = ch.to_roman_int();
        (acc + if n >= prev { n } else { -n }, n)
    });
    res.0
}

pub fn roman_to_int2(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("III", 3; "case 1")]
    #[test_case("LVIII", 58; "case 2")]
    #[test_case("MCMXCIV", 1994; "case 3")]
    fn test_roman_to_int(s: &str, expected: i32) {
        let result = roman_to_int(s.to_string());
        assert_eq!(result, expected);
    }
}

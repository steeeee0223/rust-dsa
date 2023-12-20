pub fn max_profit(prices: Vec<i32>) -> i32 {
    let res = prices
        .windows(2)
        .fold((0, 0), |(mut profit, mut curr), prices| {
            curr += prices[1] - prices[0];
            if curr > profit {
                profit = curr
            }
            if curr < 0 {
                curr = 0
            }
            (profit, curr)
        });
    res.0
}

pub fn max_profit2(prices: Vec<i32>) -> i32 {
    let res = prices
        .iter()
        .fold((0, i32::MAX), |(mut max_profit, mut cost), price| {
            cost = i32::min(*price, cost);
            max_profit = i32::max(max_profit, price - cost);
            (max_profit, cost)
        });
    res.0
}

pub fn max_profit3(prices: Vec<i32>) -> i32 {
    let mut mx = 0;
    let mut b = i32::MAX;
    prices.into_iter().for_each(|value| {
        mx = i32::max(mx, value - b);
        b = i32::min(b, value);
    });
    mx
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[7, 1, 5, 3, 6, 4], 5; "case 1")]
    #[test_case(&[7, 6, 4, 3, 1], 0; "case 2")]
    fn test_max_profit(prices: &[i32], expected: i32) {
        let result = max_profit3(prices.to_vec());
        assert_eq!(result, expected);
    }
}

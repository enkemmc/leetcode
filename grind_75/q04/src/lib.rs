use std::cmp::{min, max};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest_left = prices[0];
        let mut biggest_diff = 0;
        for &price in &prices[1..]{
            if price > lowest_left {
                biggest_diff = max(price - lowest_left, biggest_diff);
            }
            lowest_left = min(price, lowest_left);
        }
        
        biggest_diff
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn first() {
        assert_eq!(true, true);
    }
}

struct Solution;

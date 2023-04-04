impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let a = (high - low) / 2;
        if low % 2 == 1 || high % 2 == 1 {
            a + 1
        } else {
            a
        }
    }
}

// 3 4 5 6 7 = 3
// 2 3 4 5 6 = 2

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Solution;

const MOD: u64 = 1000000000 + 7;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (2..=n).fold(1_u64, |acc, x| {
            ((acc << (32 - x.leading_zeros())) + x as u64) % MOD
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn first() {
        let input = 1;
        let ans = Solution::concatenated_binary(input);
        let expected = 1;
        assert_eq!(expected, ans);
    }

    fn second() {
        let input = 3;
        let ans = Solution::concatenated_binary(input);
        let expected = 27;
        assert_eq!(expected, ans);
    }

    #[test]
    fn third() {
        let input = 42;
        let ans = Solution::concatenated_binary(input);
        let expected = i32::MAX;
        assert_eq!(expected, ans);
    }
}

struct Solution;

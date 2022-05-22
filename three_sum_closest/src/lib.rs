use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut seen = HashSet::new();

        for num in nums {
            seen.insert(num);
        }

        2
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let ans = Solution::three_sum_closest(nums, target);
        let expected = 2;
        assert_eq!(ans, target);
    }
}

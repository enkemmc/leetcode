//https://leetcode.com/problems/4sum/
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {}
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn second() {
        let target = 0;
        let nums = vec![2, 2, 2, 2, 2];
        let ans = Solution::four_sum(nums, target);
        let expected = vec![vec![2, 2, 2, 2]];
        assert_eq!(expected, ans);
    }

    #[test]
    fn first() {
        let target = 0;
        let nums = vec![1, 0, -1, 0, -2, 2];
        let ans = Solution::four_sum(nums, target);
        let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        assert_eq!(expected, ans);
    }
}

struct Solution;

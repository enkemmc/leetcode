// https://leetcode.com/problems/subsets
struct Solution;

impl Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combos = vec![];
        let mut combo = vec![];
        Self::helper(&nums, &mut combos, &mut combo);
        combos.push(combo);
        combos
    }

    fn helper(nums: &[i32], combos: &mut Vec<Vec<i32>>, combo: &mut Vec<i32>) {
        if nums.len() > 0 {
            for i in 0..nums.len() {
                combo.push(nums[i]);
                combos.push(combo.clone());
                Self::helper(&nums[i + 1..], combos, combo);
                combo.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn first() {
        let nums = vec![1, 2, 3];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let ans = Solution::subsets(nums);
        assert_eq!(
            ans.into_iter().collect::<HashSet<Vec<i32>>>(),
            expected.into_iter().collect::<HashSet<Vec<i32>>>()
        );
    }
}

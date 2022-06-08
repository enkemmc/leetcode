use std::collections::HashSet;
struct Solution;
// time:
// sort - logN

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut combos = HashSet::new();
        Self::helper(&mut combos, &nums, &mut Vec::with_capacity(3), 0);
        combos.into_iter().collect::<Vec<Vec<i32>>>()
    }
    fn helper(combos: &mut HashSet<Vec<i32>>, nums: &[i32], combo: &mut Vec<i32>, sum: i32) {
        if combo.len() == 3 {
            if sum == 0 {
                combos.insert(combo.clone());
            }
        } else if !nums.is_empty() {
            for i in 0..nums.len() {
                if nums[i] + sum > 0 {
                    return;
                } else {
                    combo.push(nums[i]);
                    Self::helper(combos, &nums[i + 1..], combo, sum + nums[i]);
                    combo.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let ans = Solution::three_sum(nums);
        assert_eq!(expected, ans);
    }
}

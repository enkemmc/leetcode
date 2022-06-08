impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let nums: Vec<i32> = (1..=9).into_iter().collect();
        let mut combos = vec![];
        Self::helper(&mut combos, &nums, n, k, &mut vec![], 0);

        combos
    }

    fn helper(
        combos: &mut Vec<Vec<i32>>,
        nums: &[i32],
        target: i32,
        limit: i32,
        combo: &mut Vec<i32>,
        sum: i32,
    ) {
        if combo.len() == limit as usize {
            if sum == target {
                println!("{:?} sum == {}", combo, sum);
                combos.push(combo.clone());
            }
        } else {
            for i in 0..nums.len() {
                combo.push(nums[i]);
                Self::helper(combos, &nums[i + 1..], target, limit, combo, sum + nums[i]);
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
        let k = 3;
        let n = 7;
        let expected = vec![vec![1, 2, 4]];
        let ans = Solution::combination_sum3(k, n);
        assert_eq!(expected, ans);
    }
}

struct Solution;

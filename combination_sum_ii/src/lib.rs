struct Solution;
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut combos = vec![];
        Self::helper(&candidates, target, &mut vec![], 0, &mut combos);
        combos.into_iter().collect()
    }

    fn helper(
        candidates: &[i32],
        target: i32,
        combo: &mut Vec<i32>,
        curr_val: i32,
        combos: &mut Vec<Vec<i32>>,
    ) {
        if candidates.len() == 0 || curr_val == target {
            if curr_val == target {
                combos.push(combo.clone());
            }
        } else {
            for i in 0..candidates.len() {
                if i > 0 && candidates[i] == candidates[i - 1] {
                    continue;
                }
                if curr_val + candidates[i] <= target {
                    combo.push(candidates[i]);
                    Self::helper(
                        &candidates[i + 1..],
                        target,
                        combo,
                        curr_val + candidates[i],
                        combos,
                    );
                    combo.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
            .into_iter()
            .collect::<HashSet<Vec<i32>>>();
        let ans = Solution::combination_sum2(candidates, target)
            .into_iter()
            .collect::<HashSet<Vec<i32>>>();
        assert_eq!(expected, ans);
    }
}

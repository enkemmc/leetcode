use std::collections::HashSet;

struct Solution;
impl Solution {
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combos = HashSet::new();
        Self::helper(&candidates, &mut combos, target, &mut vec![], 0);
        combos.into_iter().collect()
    }

    fn _helper(
        candidates: &[i32],
        combos: &mut HashSet<Vec<i32>>,
        target: i32,
        combo: &mut Vec<i32>,
        curr_val: i32,
    ) {
        if candidates.len() == 0 || curr_val == target {
            if curr_val == target {
                if !combos.contains(combo) {
                    combos.insert(combo.clone());
                }
            }
        } else {
            let num = candidates[0];
            // left
            Self::helper(&candidates[1..], combos, target, combo, curr_val);

            // right
            if curr_val + candidates[0] <= target {
                combo.push(num);
                Self::helper(
                    &candidates[1..],
                    combos,
                    target,
                    combo,
                    curr_val + candidates[0],
                );
                combo.pop();
            }

            // sideways
            if curr_val + candidates[0] <= target {
                combo.push(num);
                Self::helper(candidates, combos, target, combo, curr_val + candidates[0]);
                combo.pop();
            }
        }
    }

    fn helper(
        candidates: &[i32],
        combos: &mut HashSet<Vec<i32>>,
        target: i32,
        combo: &mut Vec<i32>,
        curr_val: i32,
    ) {
        if curr_val == target || candidates.len() == 0 {
            if !combos.contains(combo) {
                combos.insert(combo.clone());
            }
        } else {
            for i in 0..candidates.len() {
                if curr_val + candidates[i] <= target {
                    combo.push(candidates[i]);
                    Self::helper(
                        &candidates[i..],
                        combos,
                        target,
                        combo,
                        curr_val + candidates[i],
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
        let candidates = vec![2, 3, 5];
        let target = 8;
        let ans = Solution::combination_sum(candidates, target);
        let ans = ans.into_iter().collect::<HashSet<Vec<i32>>>();
        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
            .into_iter()
            .collect::<HashSet<Vec<i32>>>();
        assert_eq!(expected, ans);
    }
}

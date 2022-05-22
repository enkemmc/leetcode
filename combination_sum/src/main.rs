//https://leetcode.com/problems/combination-sum/
fn main() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let output = Solution::combination_sum(candidates, target);

    println!("{:?}", output);
}

struct Solution;
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combos = vec![];
        candidates.sort();
        Self::dfs(&candidates, target, vec![], 0, &mut combos, 0);
        combos
    }
    fn dfs(
        candidates: &[i32],
        target: i32,
        combo: Vec<i32>,
        current_sum: i32,
        combos: &mut Vec<Vec<i32>>,
        index: usize,
    ) {
        if current_sum == target {
            combos.push(combo);
        } else {
            for &num in &candidates[index..] {
                if current_sum + num <= target {
                    let mut combo = combo.clone();
                    combo.push(num);
                    Self::dfs(
                        candidates,
                        target,
                        combo,
                        current_sum + num,
                        combos,
                        index + 1,
                    );
                }
            }
        }
    }
}

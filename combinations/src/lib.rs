struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combos = vec![];
        let mut combo = vec![];
        Self::helper(1, n as usize, k as usize, &mut combos, &mut combo);
        combos
    }

    fn helper(i: usize, n: usize, k: usize, combos: &mut Vec<Vec<i32>>, combo: &mut Vec<i32>) {
        if combo.len() == k {
            combos.push(combo.clone());
        } else {
            for i in i..=n {
                combo.push(i as i32);
                Self::helper(i + 1, n, k, combos, combo);
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
        let n = 4;
        let k = 2;
        let ans = Solution::combine(n, k);
        let expected = vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ];
        assert_eq!(
            expected.into_iter().collect::<HashSet<Vec<i32>>>(),
            ans.into_iter().collect::<HashSet<Vec<i32>>>()
        );
    }
}

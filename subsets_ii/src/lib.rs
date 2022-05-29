use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn subsets_with_dup_1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut combos = HashSet::new();
        let mut combo = vec![];
        Self::helper1(&mut combos, &mut combo, &nums);
        combos.into_iter().collect()
    }

    fn helper1(combos: &mut HashSet<Vec<i32>>, combo: &mut Vec<i32>, nums: &[i32]) {
        if nums.len() == 0 {
            combos.insert(combo.clone());
        } else {
            combo.push(nums[0]);
            Self::helper1(combos, combo, &nums[1..]);
            combo.pop();
            // left
            Self::helper1(combos, combo, &nums[1..]);
        }
    }

    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combos = HashSet::new();
        let mut combo = vec![];
        Self::helper(&mut combos, &mut combo, &nums);
        combos.insert(combo);
        combos.into_iter().collect()
    }

    fn helper(combos: &mut HashSet<Vec<i32>>, combo: &mut Vec<i32>, nums: &[i32]) {
        if nums.len() > 0 {
            for i in 0..nums.len() {
                combo.push(nums[i]);
                let mut c = combo.clone();
                c.sort();
                combos.insert(c);
                Self::helper(combos, combo, &nums[i + 1..]);
                combo.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::Solution;

    #[test]
    fn first() {
        let nums = vec![1, 2, 2];
        let expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];
        let ans = Solution::subsets_with_dup(nums);
        assert_eq!(
            ans.into_iter().collect::<HashSet<Vec<i32>>>(),
            expected.into_iter().collect::<HashSet<Vec<i32>>>()
        );
    }

    #[test]
    fn second() {
        let nums = vec![4, 4, 4, 1, 4];
        let expected = vec![
            vec![4, 4],
            vec![1],
            vec![],
            vec![1, 4],
            vec![4],
            vec![4, 1],
            vec![4, 4, 1, 4],
            vec![4, 4, 4],
            vec![4, 4, 1],
            vec![4, 1, 4],
            vec![4, 4, 4, 4],
            vec![4, 4, 4, 1],
        ];
        let ans = Solution::subsets_with_dup(nums);
        assert_eq!(
            ans.into_iter()
                .map(|mut combo| {
                    combo.sort();
                    combo
                })
                .collect::<HashSet<Vec<i32>>>(),
            expected.into_iter().collect::<HashSet<Vec<i32>>>()
        );
    }
}

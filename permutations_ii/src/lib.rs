use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut perms = HashSet::new();
        Self::helper(nums, &mut perms, vec![]);
        perms.into_iter().collect()
    }

    fn helper(nums: Vec<i32>, perms: &mut HashSet<Vec<i32>>, perm: Vec<i32>) {
        if nums.len() == 0 {
            perms.insert(perm);
        } else {
            let mut seen = HashSet::new();
            for i in 0..nums.len() {
                if seen.contains(&nums[i]) {
                    continue;
                } else {
                    seen.insert(nums[i]);
                    let mut new_nums = nums.clone();
                    let num = new_nums.remove(i);
                    let mut perm = perm.clone();
                    perm.push(num);
                    Self::helper(new_nums, perms, perm);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::HashSet;

    #[test]
    fn first() {
        let nums = vec![1, 1, 2];
        let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        let ans = Solution::permute_unique(nums);
        assert_eq!(
            ans.into_iter().collect::<HashSet<Vec<i32>>>(),
            expected.into_iter().collect::<HashSet<Vec<i32>>>()
        );
    }

    #[test]
    fn second() {
        let nums = vec![1, 2, 3];
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let ans = Solution::permute_unique(nums);
        assert_eq!(
            ans.into_iter().collect::<HashSet<Vec<i32>>>(),
            expected.into_iter().collect::<HashSet<Vec<i32>>>()
        );
    }
}

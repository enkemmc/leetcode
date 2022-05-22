use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut combos = HashSet::new();
        Self::helper(&mut combos, &nums, vec![]);
        combos.into_iter().collect::<Vec<Vec<i32>>>()
    }

    fn helper(combos: &mut HashSet<Vec<i32>>, nums: &[i32], mut combo: Vec<i32>) {
        if nums.len() == 0 {
            // check if combo len > 1
            if combo.len() > 1 {
                combos.insert(combo);
            }
            return;
        } else {
            // check to see if last num <= this num

            //left
            Self::helper(combos, &nums[1..], combo.clone()); // btree left

            //right
            let num = nums[0];
            if let Some(&last) = combo.last() {
                if num >= last {
                    combo.push(num);
                }
            } else {
                combo.push(num);
            }
            Self::helper(combos, &nums[1..], combo); // btree left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn first() {
        let nums = vec![4, 6, 7, 7];
        let expected = vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
        ];
        let ans = Solution::find_subsequences(nums);
        assert_eq!(ans, expected);
    }
}

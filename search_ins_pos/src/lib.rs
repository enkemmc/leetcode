use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        let mut i = 0;
        // exit conditions are:
        //      found target
        //      i is out of bonuds of l & r
        while i >= l && i <= r {
            match nums[i as usize].cmp(&target) {
                Ordering::Equal => break,
                Ordering::Less => {
                    // go right
                    l = i+1;
                    i = l+ (r-l)/2;
                },
                Ordering::Greater => {
                // go left
                    r = i-1;
                    i = l + (r-l)/2;
                },
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn first() {
        let nums = vec![1,3,5,6];
        let target = 5;
        let expected = 2;
        let ans = Solution::search_insert(nums, target);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let nums = vec![1,3,5,6];
        let target = 2;
        let expected = 1;
        let ans = Solution::search_insert(nums, target);
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let nums = vec![1,3,5,6];
        let target = 7;
        let expected = 4;
        let ans = Solution::search_insert(nums, target);
        assert_eq!(ans, expected);
    }
}

struct Solution;

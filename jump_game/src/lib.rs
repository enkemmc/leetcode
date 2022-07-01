use std::cmp::max;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest = nums[0] as usize; // maximum reachable index
        let len = nums.len();
        for i in 0..len {
            if farthest >= len - 1 {
                // reached the end
                return true;
            } else if i > farthest {
                // got stuck on a 0
                return false;
            } else {
                farthest = max(farthest, i + nums[i] as usize);
            }
        }

        farthest >= len - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let nums = vec![2, 3, 1, 1, 4];
        let expected = true;
        let ans = Solution::can_jump(nums);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let nums = vec![3, 2, 1, 0, 4];
        let expected = false;
        let ans = Solution::can_jump(nums);
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let nums = vec![0];
        let expected = true;
        let ans = Solution::can_jump(nums);
        assert_eq!(ans, expected);
    }

    #[test]
    fn fourth() {
        let nums = vec![0, 2, 3];
        let expected = false;
        let ans = Solution::can_jump(nums);
        assert_eq!(ans, expected);
    }

    #[test]
    fn fifth() {
        let nums = vec![1, 2, 3];
        let expected = true;
        let ans = Solution::can_jump(nums);
        assert_eq!(ans, expected);
    }
    #[test]
    fn sixth() {
        let nums = vec![3, 0, 8, 2, 0, 0, 1];
        let expected = true;
        let ans = Solution::can_jump(nums);
        assert_eq!(ans, expected);
    }
}

struct Solution;

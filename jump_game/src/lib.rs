impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut right_most = nums[0] as usize;
        if nums.len() == 1 && nums[0] == 0 {
            return true;
        }

        for i in 0..len {
            if i + nums[i] as usize > right_most {
                right_most = i + nums[i] as usize;
            }
            if i == right_most {
                return false;
            }
            if right_most == len - 1 {
                return true;
            }
        }
        right_most >= len - 1
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
}

struct Solution;

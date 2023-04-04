use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        if nums[l] == target {
            return l as i32;
        } else if nums[r] == target {
            return r as i32;
        }
        let mut mid;
        while l < r {
            if target > nums[r] || target < nums[l] {
                break;
            }
            mid = (r + l) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => l = mid + 1,
                Ordering::Greater => r = mid - 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let nums = vec![-1,0,3,5,9,12];
        let target = 9;
        let ans = Solution::search(nums, target);
        let expected = 4;
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let nums = vec![-1,0,3,5,9,12];
        let target = 2;
        let ans = Solution::search(nums, target);
        let expected = -1;
        assert_eq!(ans, expected);
    }
}

struct Solution;

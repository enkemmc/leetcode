struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut r = nums.len() - 1;
        let mut l = 0; 
        let mut mun = nums[l];

        while l <= r {
            if nums[l] > target && nums[r] < target {
                return -1
            } else {
                if nums[l] == target {
                    return l as i32;
                } else if nums[r] == target {
                    return r as i32;
                }
                l += 1;
                r -= 1;
            }
        }

        -1
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 0;
        let ans = Solution::search(nums, target);
        let expected = 4;
        assert_eq!(ans, 4);
    }
}

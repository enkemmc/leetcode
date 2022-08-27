impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let len_n = nums.len();
        nums.sort();
        
        let mut closest = nums[0] + nums[1] + nums[len_n -1];
        
        for c in 0..len_n - 2{
            let mut lo = c + 1;
            let mut hi = len_n - 1;
            
            while lo < hi{
                let sum = nums[c] + nums[lo] + nums[hi];
                
                if sum < target{
                    while lo < hi && nums[lo] == nums[lo + 1]{
                        lo += 1;
                    }
                    lo += 1;
                } else if sum > target{
                    while lo < hi && nums[hi - 1] == nums[hi]{
                        hi -= 1;
                    }
                    hi -= 1;
                } else {
                    return sum;
                }
                
                if (target - sum).abs() < (target - closest).abs(){
                    closest = sum;
                }
            }
        }
        
        closest
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let nums = vec![-1,2,1,-4];
        let target = 1;
        let ans = Solution::three_sum_closest(nums, target);
        let expected = 2;
        assert_eq!(expected, ans);
    }
}

struct Solution;

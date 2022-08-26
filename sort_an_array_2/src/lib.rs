impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::_qs(&mut nums);
        nums
    }

    // return the position of the pivot
    fn _qs(nums: &mut [i32]) {
        let len = nums.len();
        if len > 1 {
            // partition start
            let pivot_index = len - 1;
            let pivot = nums[pivot_index];
            let mut l: isize = -1;
            let mut r = 0;

            while r < pivot_index {
                if nums[r] > pivot {
                    // do not do shit
                    r += 1;
                } else {
                    l += 1;
                    nums.swap(l as usize, r);
                    r += 1;
                }
            }
            l += 1;
            nums.swap(l as usize, pivot_index);
            // partition stop

            Self::_qs(&mut nums[..l as usize]);
            Self::_qs(&mut nums[1+l as usize..]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn first() {
        let nums = vec![1,2,3,3,2,1];
        let ans = Solution::sort_array(nums.clone());
        let mut expected = nums;
        expected.sort();
        assert_eq!(expected, ans);
    }

}

struct Solution;

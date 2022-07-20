use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let heap = BinaryHeap::from(nums);
        heap.into_vec()[k as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let nums = vec![3,2,1,5,6,4];
        let k = 2;
        let expected = 5;
        let ans = Solution::find_kth_largest(nums, k);
        assert_eq!(expected, ans);
    }
}

struct Solution;

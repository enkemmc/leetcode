use std::cmp::{Ordering, min};
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let is_even = len % 2 == 0;
        let target = (len - 1) / 2;

        let mut n1i = 0;
        let mut n2i = 0;


        while (n1i + n2i) < target {
            increment(&mut n1i, &mut n2i, &nums1, &nums2);
        }

        if is_even {
            let first = f64::from(min(*nums1.get(n1i).unwrap_or(&i32::MAX), *nums2.get(n2i).unwrap_or(&i32::MAX)));
            increment(&mut n1i, &mut n2i, &nums1, &nums2);
            let second = f64::from(min(*nums1.get(n1i).unwrap_or(&i32::MAX), *nums2.get(n2i).unwrap_or(&i32::MAX)));
            (first + second) / 2.0
        } else {
            f64::from(min(*nums1.get(n1i).unwrap_or(&i32::MAX), *nums2.get(n2i).unwrap_or(&i32::MAX)))
        }
    }
}

pub fn increment(n1i: &mut usize, n2i: &mut usize, nums1: &[i32], nums2: &[i32]){
    match nums1.get(*n1i).unwrap_or(&i32::MAX).cmp(&nums2.get(*n2i).unwrap_or(&i32::MAX)) {
        Ordering::Greater => *n2i += 1,
        Ordering::Equal =>  *n2i += 1, // ?
        Ordering::Less => *n1i += 1, 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let nums1 = vec![1,3];
        let nums2 = vec![2];
        let ans = f64::from(2);
        let output = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, ans);
    }

    #[test]
    fn test2() {
        let nums1 = vec![1,2];
        let nums2 = vec![3,4];
        let ans = (f64::from(2) + f64::from(3)) / f64::from(2);
        let output = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, ans);
    }

    #[test]
    fn test3() {
        let nums1 = vec![];
        let nums2 = vec![1];
        let ans = 1.0f64;
        let output = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, ans);
    }
}

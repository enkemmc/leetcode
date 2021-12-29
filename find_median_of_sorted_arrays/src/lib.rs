use std::borrow::BorrowMut;

struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let max_moves = if len % 2 == 0 {
            (len / 2) - 1
        } else {
            len / 2
        };
        let mut moves = 0;
        let (mut left, mut right) = (RowWrapper::new(nums1), RowWrapper::new(nums2));

        while moves < max_moves {
            increment(&mut left, &mut right);

            moves += 1;
        }

        get_median(left, right)
    }

}

fn increment(left: &mut RowWrapper, right: &mut RowWrapper){
    if left.has_smaller(right) { 
        left.inc_left() 
    } else { 
        right.inc_left() 
    }
    if left.has_bigger(right) { 
        left.inc_right() 
    } else { 
        right.inc_right() 
    }
}

fn get_median(left: RowWrapper, right: RowWrapper) -> f64 {
    if left.is_empty {
        right.get_median()
    } else if right.is_empty {
        left.get_median()
    } else {
        (left.get_median() + right.get_median()) / 2f64
    }
}

struct RowWrapper {
    arr: Vec<i32>,
    li: usize,
    ri: usize,
    is_empty: bool
}

impl RowWrapper {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            ri: if !nums.is_empty() { nums.len() - 1 } else { 0 },
            li: 0,
            is_empty: nums.is_empty(),
            arr: nums,
        }
    }

    fn has_smaller(&self, other: &RowWrapper) -> bool {
        if self.is_empty {
            false
        } else {
            if other.is_empty {
                true
            } else {
                self.arr[self.li] < other.arr[other.li]
            }
        }
    }

    fn has_bigger(&self, other: &RowWrapper) -> bool {
        if self.is_empty {
            false
        } else {
            if other.is_empty {
                true
            } else {
                self.arr[self.ri] > other.arr[other.ri]
            }
        }
    }

    fn calc_empty(&self) -> bool {
        self.ri < self.li || self.li > self.ri
    }

    fn inc_left(&mut self){
        if self.li == self.ri {
            self.is_empty = true;
        } else {
            self.li += 1;
        }

        if self.calc_empty() {
            self.is_empty = true;
        }
    }

    fn inc_right(&mut self){
        if self.ri == 0 {
            self.is_empty = true;
        } else {
            self.ri -= 1;
        }

        if self.calc_empty() {
            self.is_empty = true;
        }
    }

    fn get_median(&self) -> f64 {
        if self.li != self.ri {
            f64::from(self.arr[self.li] + self.arr[self.ri]) / f64::from(2)
        } else {
            f64::from(self.arr[self.li])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test4() {
        let nums1 = vec![1,3];
        let nums2 = vec![2,7];
        let ans = (5f64) / f64::from(2);
        let output = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, ans);
    }

    #[test]
    fn test5() {
        let nums1 = vec![3];
        let nums2 = vec![-2, -1];
        let ans = -1f64;
        let output = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, ans);
    }

    #[test]
    fn test6() {
        let nums1 = vec![2, 3, 4, 5];
        let nums2 = vec![];
        let ans = 7f64 / 2f64;
        let output = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(output, ans);
    }
}

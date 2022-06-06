impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() > 1 {
            fn quick_sort(nums: &mut [i32]) {
                if nums.len() > 0 {
                    let mid = partition(nums);
                    quick_sort(&mut nums[..mid]);
                    quick_sort(&mut nums[mid + 1..]);
                }
            }
            fn partition(nums: &mut [i32]) -> usize {
                let pivot = nums.len() / 2;
                let target = nums[pivot];
                nums.swap(pivot, nums.len() - 1);
                let mut l: i32 = -1;
                let mut r = 0;
                while r < nums.len() {
                    if nums[r] < target {
                        l += 1;
                        nums.swap(l as usize, r);
                    }
                    r += 1;
                }
                l += 1;
                nums.swap(l as usize, nums.len() - 1);
                l as usize
            }
            quick_sort(&mut nums);
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second() {
        let s = String::new();
        let input = vec![5, 1, 1, 2, 0, 0];
        let mut expected = input.clone();
        expected.sort();
        let ans = Solution::sort_array(input);
        assert_eq!(ans, expected);
    }

    #[test]
    fn first() {
        let input = vec![5, 2, 3, 1];
        let mut expected = input.clone();
        expected.sort();
        let ans = Solution::sort_array(input);
        assert_eq!(ans, expected);
    }
}

struct Solution;

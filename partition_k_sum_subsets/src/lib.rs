// 698. Partition to K Equal Sum Subsets
// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
struct Solution;
impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        let mut remaining_space = [sum / k; 16]; // k <= 16.  ideally 16 would be k, but rust 8)

        Self::helper(&mut remaining_space, sum, k as usize, &nums)
    }

    fn helper(remaining_space: &mut [i32; 16], target: i32, k: usize, nums: &[i32]) -> bool {
        if nums.len() == 0 {
            // if we reach this point, we've been able to fit all of the numbers into their buckets
            // all buckets are completely empty
            true
        } else {
            let num = nums[0];
            for i in 0..k {
                if remaining_space[i] >= num {
                    // if there are remaining nums && adding the next smallest number (i+1 since sorted) won't cause overflow
                    remaining_space[i] -= num;
                    if Self::helper(remaining_space, target, k, &nums[1..]) {
                        return true;
                    }
                    remaining_space[i] += num;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let nums = vec![4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        let expected = true;
        let ans = Solution::can_partition_k_subsets(nums, k);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let nums = vec![1, 2, 3, 4];
        let k = 3;
        let expected = false;
        let ans = Solution::can_partition_k_subsets(nums, k);
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let nums = vec![
            7628, 3147, 7137, 2578, 7742, 2746, 4264, 7704, 9532, 9679, 8963, 3223, 2133, 7792,
            5911, 3979,
        ];

        let k = 6;
        let expected = false;
        let ans = Solution::can_partition_k_subsets(nums, k);
        assert_eq!(ans, expected);
    }
}

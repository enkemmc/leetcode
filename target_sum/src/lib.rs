struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut count = 0;
        // 2001 == 1000+1000 no more than 20 deep.
        // each level's bool array represents whether the sum has been seen before
        // use an offset of the highest number possible to ensure the sum stays positive
        let offset = nums.iter().sum::<i32>();

        let mut depth_to_sum_tracker = [[false; 2001]; 20];
        Self::helper(
            &nums,
            &mut count,
            target + offset,
            offset,
            0,
            &mut depth_to_sum_tracker,
        );
        count
    }

    fn helper(
        nums: &[i32],
        count: &mut i32,
        target: i32,
        sum: i32,
        depth: usize,
        depth_to_sum_tracker: &mut [[bool; 2001]; 20],
    ) {
        if depth == nums.len() {
            if target == sum {
                *count += 1;
            }
        } else {
            // check to see if we've already seen this depth + sum pair
            // if we have, that tree already captured all combinations, so just return to save time
            if depth_to_sum_tracker[depth][sum as usize] {
                return;
            } else {
                depth_to_sum_tracker[depth][sum as usize] = true;
            }

            Self::helper(
                nums,
                count,
                target,
                sum + nums[depth],
                depth + 1,
                depth_to_sum_tracker,
            );
            Self::helper(
                nums,
                count,
                target,
                sum - nums[depth],
                depth + 1,
                depth_to_sum_tracker,
            );
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn first() {
        let dp = (&mut [0; 2001], &mut [0; 2001]);
        let nums = vec![1, 1, 1, 1, 1];
        let target = 3;
        let ans = Solution::find_target_sum_ways(nums, target);
        let expected = 5;
        assert_eq!(expected, ans);
    }
}

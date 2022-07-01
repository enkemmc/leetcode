use std::cmp::{max, min};
// jump locations are valued at nums[i] - i
// if two jump locations have the same value, it means their potential jump locations are equal
// nums[i] == 0 is never a valid jump location, unless i == len - 1 (aka it is the last node)
//
// solution can be n^2

impl Solution {
    // look at all jump locations
    // if one is the end, pick it
    // otherwise, calculate the one with the most potential
    // an index's potential is defined as nums[i] + i
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut jumps = 0;
        let mut i = 0;
        let mut potential = None; // the index of the jump with the most potential
        while i < len - 1 {
            potential = None;
            // iterate through the indexes that are reachable from i
            let end = min(i + 1 + nums[i] as usize, len);
            if end - 1 >= len - 1 {
                jumps += 1;
                break;
            }
            for x in min(i + 1, len - 1)..end {
                // x is a true index not an offset
                // find the spot that will have the most potential
                if let Some(pi) = potential {
                    potential = Some(Self::better_potential(pi, x, &nums));
                } else {
                    if nums[x] != 0 {
                        potential = Some(x);
                    }
                }
            }
            i = potential.take().unwrap();
            jumps += 1;
        }
        jumps
    }

    fn better_potential(i1: usize, i2: usize, nums: &[i32]) -> usize {
        if i1 + nums[i1] as usize > i2 + nums[i2] as usize {
            i1
        } else {
            i2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let input = vec![2, 3, 1, 1, 4];
        let expected = 2;
        let output = Solution::jump(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn second() {
        let input = vec![2, 3, 0, 1, 4];
        let expected = 2;
        let output = Solution::jump(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn third() {
        let input = vec![1, 2, 3];
        let expected = 2;
        let output = Solution::jump(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn fourth() {
        let input = vec![2, 3, 1];
        let expected = 1;
        let output = Solution::jump(input);
        assert_eq!(expected, output);
    }
}

struct Solution;

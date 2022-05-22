use std::cell::RefCell;
use std::rc::Rc;
struct Solution{

}

impl Solution {

    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let (mut l, mut r) = (0, 0);
        let mut leap_potential = 0;

        while r < nums.len() - 1 {
            leap_potential = 0;
            for i in l..=r {
                leap_potential = std::cmp::max(leap_potential, i + nums[i] as usize); // its current index value + its potential == its total potential
            }
            l = r + 1;
            r = leap_potential;
            jumps += 1;
        }


        jumps
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = vec![2,3,1,1,4];
        let expected = 2;
        let output = Solution::jump(input);
        assert_eq!(expected, output);
    }
}

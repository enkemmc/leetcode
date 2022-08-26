use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut n_as_s = n.to_string().chars().collect::<Vec<char>>();
        n_as_s.sort();

        for i in 0..30 {
            let mut s =2i32.pow(i as u32).to_string().chars().collect::<Vec<char>>();
            s.sort();
            if s == n_as_s {
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let input = 1;
        let expected = true;
        let ans = Solution::reordered_power_of2(input);
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let input = 10;
        let expected = false;
        let ans = Solution::reordered_power_of2(input);
        assert_eq!(expected, ans);
    }


}

struct Solution;

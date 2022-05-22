use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

//* [1, 1, 2, 2, 2]
//* [1, 3, 3, 4, 4, 4, 5, 7] // 8
struct Solution;
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }

        let len = sum / 4;
        let mut side_space = [len; 4];
        return Self::dfs(&mut side_space, len);
    }

    fn dfs(side_space: &mut [i32; 4], len: i32) -> bool {
        true
    }
}

mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let matchsticks = vec![1, 1, 2, 2, 2];
        let expected = true;
        let ans = Solution::makesquare(matchsticks);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second_test() {
        let matchsticks = vec![1, 1, 3, 3, 4, 4, 4, 5, 7];
        let expected = true;
        let ans = Solution::makesquare(matchsticks);
        assert_eq!(ans, expected);
    }
}

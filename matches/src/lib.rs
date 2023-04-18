struct Solution;
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            false
        } else {
            let side_len = sum / 4;
            for stick in matchsticks.iter() {
                if stick > &side_len {
                    return false;
                }
            }
            // let mut side_space = [side_len; 4];
            Self::helper(&mut [side_len; 4], side_len, &matchsticks)
        }
    }

    fn helper(side_space: &mut [i32; 4], side_len: i32, matchsticks: &[i32]) -> bool {
        if matchsticks.len() == 0 {
            // side_space.iter().sum::<i32>() == side_len * 4 // only not true when ??
            side_space.iter().all(|&s| s == 0)
        } else {
            let stick = matchsticks[0];
            for i in 0..4 {
                if side_space[i] >= stick {
                    side_space[i] -= stick;
                    if Self::helper(side_space, side_len, &matchsticks[1..]) {
                        return true;
                    }
                    side_space[i] += stick; // while this stick fits in this side, it will cause a fail later
                }
            }
            false
        }
    }

    fn backtrack(matches: &mut [i32], matchsticks: &[i32], match_index: usize) -> bool {
        if match_index == matchsticks.len() {
            return matches.iter().all(|m| *m == 0);
        }
        for j in 0..4 {
            if matches[j] - matchsticks[match_index] >= 0 {
                matches[j] -= matchsticks[match_index];
                if Solution::backtrack(matches, matchsticks, match_index + 1) {
                    return true;
                }
                matches[j] += matchsticks[match_index];
            }
        }
        false
    }
    pub fn makesquare2(matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            return false;
        }

        let avg = sum / 4;
        let mut matches = vec![avg; 4];
        Solution::backtrack(&mut matches, &matchsticks, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let matchsticks = vec![1, 1, 2, 2, 2];
        let output = Solution::makesquare(matchsticks);
        let expected = true;
        assert_eq!(expected, output);
    }

    #[test]
    fn second() {
        let matchsticks = vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3];
        let output = Solution::makesquare(matchsticks);
        let expected = true;
        assert_eq!(expected, output);
    }

    #[test]
    fn third() {
        let matchsticks = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6];
        let output = Solution::makesquare(matchsticks);
        let expected = false;
        assert_eq!(expected, output);
    }

    #[test]
    fn thing() {
        let mut s = "asdf".to_string();
        s.replacen(j, to, count)
    }
}

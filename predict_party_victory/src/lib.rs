use std::collections::VecDeque;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut queue = VecDeque::with_capacity(senate.len());
        let mut dire_to_remove = 0;
        let mut radiant_to_remove = 0;
        let mut dire_remaining = 0;
        let mut radiant_remaining = 0;
        if senate.len() == 1 {
            if senate.chars().next().unwrap() == 'D' {
                return "Dire".to_string();
            } else {
                return "Radiant".to_string();
            }
        }

        for c in senate.chars() {
            match c {
                'D' => {
                    if dire_to_remove > 0 {
                        dire_to_remove -= 1;
                    } else {
                        dire_remaining += 1;
                        radiant_to_remove += 1;
                        queue.push_back('D');
                    }
                },
                'R' => {
                    if radiant_to_remove > 0 {
                        radiant_to_remove -= 1;
                    } else {
                        radiant_remaining += 1;
                        dire_to_remove += 1;
                        queue.push_back('R');
                    }
                },
                _ => unreachable!()
            }
        }

        while (dire_remaining - dire_to_remove) > 0 && (radiant_remaining - radiant_to_remove) > 0 {
            match queue.pop_front().unwrap() {
                'D' => {
                    if dire_to_remove > 0 {
                        dire_to_remove -= 1;
                        dire_remaining -= 1;
                    } else {
                        radiant_to_remove += 1;
                        queue.push_back('D');
                    }
                },
                'R' => {
                    if radiant_to_remove > 0 {
                        radiant_to_remove -= 1;
                        radiant_remaining -= 1;
                    } else {
                        dire_to_remove += 1;
                        queue.push_back('R');
                    }
                },
                _ => unreachable!()
            }
        }

        if dire_remaining - dire_to_remove  > 0 {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let senate = "RD".to_string();
        let ans = Solution::predict_party_victory(senate);
        let expected = "Radiant".to_string();
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let senate = "DRDRR".to_string();
        let ans = Solution::predict_party_victory(senate);
        let expected = "Dire".to_string();
        assert_eq!(ans, expected);
    }
}

struct Solution;

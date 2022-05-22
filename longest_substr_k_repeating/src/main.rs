use std::collections::{HashMap, HashSet};
fn main() {}

struct Solution;
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut longest = 0;
        for i in 0..chars.len() {
            let mut tracker = Tracker::new(k);
            tracker.add(&chars[i..]);
            let len = tracker.get_longest();
            if len > longest {
                longest = len;
            }
        }

        longest as i32
    }
}

struct Tracker {
    map: HashMap<char, usize>,
    min: usize,
    longest: usize,
    missing_chars: HashSet<char>,
}

impl Tracker {
    fn new(k: i32) -> Self {
        Self {
            map: HashMap::new(),
            longest: 0,
            min: k as usize,
            missing_chars: HashSet::new(),
        }
    }

    fn add(&mut self, chars: &[char]) {
        let mut is_valid = true;

        let mut ch;
        for i in 0..chars.len() {
            ch = chars[i];
            // increment the count
            *self.map.entry(ch.clone()).or_insert(0) += 1;

            // if the count is >= k, remove it from missing. if missing is empty, substr is valid
            if let Some(count) = self.map.get(&ch) {
                if count < &self.min {
                    self.missing_chars.insert(ch.clone());
                    is_valid = false;
                } else {
                    self.missing_chars.remove(&ch);
                    if self.missing_chars.len() == 0 {
                        is_valid = true;
                        self.longest = i + 1;
                    }
                }
            }
        }
    }

    fn get_longest(&self) -> usize {
        self.longest
    }
}

mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let s = "aaabb".to_string();
        let k = 3;
        let expected = 3;
        let ans = Solution::longest_substring(s, k);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second_test() {
        let s = "ababbc".to_string();
        let k = 2;
        let expected = 5;
        let ans = Solution::longest_substring(s, k);
        assert_eq!(ans, expected);
    }
}

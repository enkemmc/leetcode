use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_charcount: HashMap<char, usize> = HashMap::new();
        for sc in s.chars() {
            let entry = s_charcount.entry(sc).or_insert(0);
            *entry += 1;
        }
        for tc in t.chars() {
            if let Some(count) = s_charcount.get_mut(&tc) {
                if *count == 0 {
                    return false;
                } else {
                    *count -= 1;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let ans = Solution::is_anagram(s, t);
        assert_eq!(ans, true);
    }
}

struct Solution;

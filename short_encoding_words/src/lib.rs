use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by(|p, c| c.len().cmp(&p.len()));
        let mut size = 0;
        // 1) if the word EXACTLY matches, remove all refs
        // 2) if the word is a substr, remove all refs
        // 3) if the word is not a substr, add it to used AND increment size
        let mut used: HashSet<&str> = HashSet::new();

        for word in &words {
            // 1
            if used.contains(word.as_str()) {
                continue;
            }

            // 2
            let mut found = false;
            'loop_1: for top_word in &used {
                if top_word.len() >= word.len() {
                    let diff = top_word.len() - word.len();

                    if &top_word[diff..] == word {
                        // substr matches
                        used.insert(word);
                        found = true;
                        break 'loop_1;
                    }
                }
            }

            // 3
            if !found {
                // not a substr
                size += word.len() + 1;
                used.insert(word);
            }
        }

        size as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn third() {
        let input: Vec<String> = vec!["time", "atime", "btime"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let ans = Solution::minimum_length_encoding(input);
        let expected = 12;
        assert_eq!(expected, ans);
    }

    #[test]
    fn first() {
        let input: Vec<String> = vec!["time", "me", "bell"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let ans = Solution::minimum_length_encoding(input);
        let expected = 10;
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let input: Vec<String> = vec!["t"].into_iter().map(|s| s.to_string()).collect();
        let ans = Solution::minimum_length_encoding(input);
        let expected = 2;
        assert_eq!(expected, ans);
    }
}

struct Solution;

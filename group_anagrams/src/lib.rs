use std::collections::HashMap;

// a = 97
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut output = HashMap::new();
        let state = [0u8;25];
        for s in strs {
            let mut state = state.clone();
            for ch in s.chars() {
                state[(ch as u8 - 97) as usize] += 1;
            }
            output.entry(state).or_insert_with(Vec::new).push(s);
        }
        output.into_values().into_iter().collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let strs = vec!["eat","tea","tan","ate","nat","bat"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expected = vec![vec!["bat"],vec!["nat","tan"],vec!["ate","eat","tea"]];
        let ans = Solution::group_anagrams(strs);
        assert_eq!(ans, expected);
    }
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by_cached_key(String::len);
        let mut hm = HashMap::new();
        let mut answer = 0;
        for word in &words {
            let max = (0..word.len())
                .filter_map(|i| hm.get(&(String::new() + &word[0..i] + &word[i + 1..])))
                .max()
                .unwrap_or(&0)
                + 1;
            hm.insert(word, max);
            answer = answer.max(max);
        }
        answer
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn first() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Solution;

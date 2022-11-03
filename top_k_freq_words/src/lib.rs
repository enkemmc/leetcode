use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_to_count = HashMap::new();
        for word in words {
            *word_to_count.entry(word).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new();
        for (word, count) in word_to_count {
            heap.push((count, word));
        }
        let mut ans = vec![];
        for _ in 0..k {
            if let Some((_count, word)) = heap.pop() {
                ans.push(word);
            }
        }
        ans.reverse();
        ans
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn first() {
        let words: Vec<String> = vec!["i","love","leetcode","i","love","coding"].into_iter().map(|s| s.to_string()).collect();
        let k = 2;
        let expected: Vec<String> = vec!["i", "love"].into_iter().map(|s| s.to_string()).collect();
        let ans = Solution::top_k_frequent(words, k);
        assert_eq!(ans, expected)
    }
}

struct Solution;

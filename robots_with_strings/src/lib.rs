use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn robot_sort(s: String) -> String {
        let mut heap = BinaryHeap::new();
        for ch in s.chars() {
            heap.push(Reverse(ch as u8));
        }

        let mut ans = Vec::with_capacity(s.len());
        while let Some(ch) = heap.pop() {
            ans.push(ch.0);
        }
        String::from_utf8(ans).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = "gregaergaergafdcbveaber".to_string();
        let ans = Solution::robot_sort(input.clone());
        let mut expected: Vec<u8> = input.chars().map(|c| c as u8).collect();
        expected.sort();
        let expected = String::from_utf8(expected).unwrap();

        assert_eq!(ans, expected);
    }
}

struct Solution;

use std::collections::VecDeque;

//sadsam
//aarsadsadsam
//r-index

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        let mut queue = VecDeque::new();
        let target_len = needle.len() - 1;
        let mut needle_chars = needle.chars().collect::<Vec<char>>();

        for (r, ch) in haystack.char_indices() {
            // try to progress items in queue
            for _ in 0..queue.len() {
                if let Some(Maybe(start)) = queue.pop_front() {
                    let curr = r - start;
                    if ch == needle_chars[curr] {
                        if r - start == target_len {
                            return start as i32;
                        } else {
                            queue.push_back(Maybe(start));
                        }
                    }
                }
            }

            // add new item to queue
            if ch == needle_chars[0] {
                if needle_chars.len() == 1 {
                    return r as i32;
                }
                queue.push_back(Maybe(r));
            }
        }

        if let Some(Maybe(start)) = queue.pop_front() {
            if haystack.len() - 1 - start == target_len {
                return start as i32;
            }
        }
        -1
    }
}

struct Maybe(usize);

#[cfg(test)]
mod tests {
    use super::*;

    fn first() {
        let needle = "a".to_string();
        let haystack = "aaa".to_string();
        let expected = 0;
        let ans = Solution::str_str(haystack, needle);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let haystack = "mississippi".to_string();
        let needle = "sippia".to_string();
        let expected = -1;
        let ans = Solution::str_str(haystack, needle);
        assert_eq!(ans, expected);
    }
}

struct Solution;

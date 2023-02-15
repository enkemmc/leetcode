use std::collections::HashMap;


impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut longest = 0;
        let mut used = HashMap::new();
        let mut l = 0;
        let bytes = s.bytes().map(|b| b as usize).collect::<Vec<usize>>();
        for r in 0..s.len() {
            if let Some(&i) = used.get(&bytes[r]) {
                // increment l until i, removing each index from used, THEN increment one further
                //println!("l{}i{}", l, i);
                while l < i {
                    used.remove(&bytes[l]);
                    l += 1;
                }
                l += 1;
                used.insert(bytes[r], r);
                //println!("23: {} {} {}", l, r, longest+1);
            } else {
                used.insert(bytes[r], r);
                longest = (r-l).max(longest);
                //println!("27: {} {} {}", l, r, longest+1);
            }
        }
        longest = (s.len()-1-l).max(longest);
        1+ longest as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let s = "abcabcbb".to_string();
        let expected = 3;
        let ans = Solution::length_of_longest_substring(s);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let s = "aginubkpmiardjlmasduzjfqnukdpiwsqehmisbwceqgnbunvxjwipowlybdhxvxdcknwlkzrchefbrazdyjsmhf".to_string();
        let expected = 7;
        let ans = Solution::length_of_longest_substring(s);
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let s = "bpfbhmipx".to_string();
        let expected = 7;
        let ans = Solution::length_of_longest_substring(s);
        assert_eq!(ans, expected);
    }

}

struct Solution;

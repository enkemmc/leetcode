use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 1 { return 1 };
        let mut l = 0;
        let mut r = 0;
        let mut longest = 0;
        let mut chars = s.char_indices().collect::<Vec<(usize, char)>>();
        let mut char_to_index_map = HashMap::new();

        for (i, ch) in &chars {
            longest = if (r - l) > longest { r - l } else { longest };
            if let Some(&prev_index) = char_to_index_map.get(ch){
                for (_, och) in &chars[l..=prev_index] {
                    char_to_index_map.remove_entry(och);
                }

                l = prev_index + 1;
                char_to_index_map.insert(ch, *i);
            } else {
                char_to_index_map.insert(ch, *i);
            }
            r += 1;
        }

        longest = if (r - l) > longest { r - l } else { longest };
        longest as i32
    }
}

 mod test {
    use super::*;
    #[test]
    fn test1(){
        let input = "abcabcbb".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test2(){
        let input = "bbbbb".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 1);
    }


    #[test]
    fn test3(){
        let input = "pwwkew".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test4(){
        let input = " ".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 1);
    }

    #[test]
    fn test5(){
        let input = "au".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 2);
    }

    #[test]
    fn test6(){
        let input = "abba".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 2);
    }
 }
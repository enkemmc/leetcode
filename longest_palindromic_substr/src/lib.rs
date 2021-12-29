use std::ops::Range;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 || s.len() == 1 { return s }
        let chars = s.chars().map(|ch| ch as u8).collect::<Vec<u8>>();
        let mut longest = 0..1;

        for (i, ch) in chars.iter().enumerate() {
            let str_rng = expand(&chars, i);
            if (longest.end - longest.start) < (str_rng.end - str_rng.start) {
                longest = str_rng;
            }
        }

        
        String::from_utf8(chars[longest].to_vec()).unwrap()
    }
}

fn expand(chars: &Vec<u8>, start_index: usize) -> Range<usize> {
    let mut start = start_index;
    let mut end = start_index;

    // handle both single and double // aka, start is one or two chars

    // single
    while 
        (start != 0 && end != chars.len() - 1) // oob check
        &&
        (chars[start-1] == chars[end+1]) // still a palindrome
        {
        start -= 1;
        end += 1;      
    }

    let single_len = start..end+1;

    start = start_index;
    end = start_index;

    if chars.get(start) == chars.get(end + 1){
        end += 1;
    }
    // double
    while 
        (start != 0 && end != chars.len() - 1) // oob check
        &&
        (chars[start-1] == chars[end+1]) // still a palindrome
        {
        start -= 1;
        end += 1;      
    }

    let double_len = start..end+1;

    if (single_len.end - single_len.start) > (double_len.end - double_len.start) {
        single_len
    } else {
        double_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = "babad".to_string();
        let expected = "bab".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }

    #[test]
    fn test2() {
        let input = "cbbd".to_string();
        let expected = "bb".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }

    #[test]
    fn test3() {
        let input = "".to_string();
        let expected = "".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }
    #[test]
    fn test4() {
        let input = "bb".to_string();
        let expected = "bb".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }

    #[test]
    fn test5() {
        let input = "tattarrattat".to_string();
        let expected = "tattarrattat".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }

    #[test]
    fn test6(){
        let input = "aacabdkacaa".to_string();
        let expected = "aca".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }

    #[test]
    fn test7(){
        let input = "ccc".to_string();
        let expected = "ccc".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }

    #[test]
    fn test8(){
        let input = "azwdzwmwcqzgcobeeiphemqbjtxzwkhiqpbrprocbppbxrnsxnwgikiaqutwpftbiinlnpyqstkiqzbggcsdzzjbrkfmhgtnbujzszxsycmvipjtktpebaafycngqasbbhxaeawwmkjcziybxowkaibqnndcjbsoehtamhspnidjylyisiaewmypfyiqtwlmejkpzlieolfdjnxntonnzfgcqlcfpoxcwqctalwrgwhvqvtrpwemxhirpgizjffqgntsmvzldpjfijdncexbwtxnmbnoykxshkqbounzrewkpqjxocvaufnhunsmsazgibxedtopnccriwcfzeomsrrangufkjfzipkmwfbmkarnyyrgdsooosgqlkzvorrrsaveuoxjeajvbdpgxlcrtqomliphnlehgrzgwujogxteyulphhuhwyoyvcxqatfkboahfqhjgujcaapoyqtsdqfwnijlkknuralezqmcryvkankszmzpgqutojoyzsnyfwsyeqqzrlhzbc".to_string();
        let expected = "sooos".to_string();
        let ans = Solution::longest_palindrome(input);

        assert_eq!(ans, expected);
    }
}


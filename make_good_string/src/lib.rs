impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack = vec![];
        for b in s.bytes() {
            if !stack.is_empty() {
                match b as i8 - *stack.last().unwrap() as i8 {
                    32 | -32 => {
                        stack.pop();
                    },
                    _ => {
                        stack.push(b);
                    }
                }
            } else {
                stack.push(b);
            }
        }

        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let s = "leEeetcode".to_string();
        let expected = "leetcode".to_string();
        let ans = Solution::make_good(s);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        //println!("{} {}", b'a', b'A');
        //println!("{} {} {}", 97u8 as char, 65u8 as char, 99u8 as char);
        let s = "abBAcC".to_string();
        let expected = "".to_string();
        let ans = Solution::make_good(s);
        assert_eq!(ans, expected);
    }

}

struct Solution;

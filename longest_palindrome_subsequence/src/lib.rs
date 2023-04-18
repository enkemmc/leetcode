impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<u8> = s.bytes().collect();
        let mut dp = vec![vec![0i32;s.len()];s.len()];
        let mut end;
        for len in 0..s.len() {
            for start in 0..s.len()-len {
                end = start+len;
                match end-start {
                    0 => dp[start][end] = 1,
                    1 => {
                        if s[start] == s[end] {
                            dp[start][end] = 2;
                        } else {
                            dp[start][end] = 1;
                        }
                    },
                    _n => {
                        if s[start] == s[end] {
                            dp[start][end] = 2 + dp[start+1][end-1];
                        } else {
                            dp[start][end] = dp[start][end-1].max(dp[start+1][end]);
                        }
                    }
                }
            }
        }
        dp[0][s.len()-1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let s = "bbbab".to_string();
        let expected = 4;
        let ans = Solution::longest_palindrome_subseq(s);
        assert_eq!(expected, ans);
    }
}

struct Solution;

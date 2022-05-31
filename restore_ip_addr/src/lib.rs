use std::cmp::min;
use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut addrs = vec![];
        let chars = s.chars().collect::<Vec<char>>();

        // [12345]
        'first: for l in 1..4 {
            'second: for m in l + 1..l + 4 {
                'third: for r in m + 1..m + 5 {
                    if l >= m {
                        break 'first;
                    } else if m >= r {
                        break 'second;
                    } else if r >= s.len() {
                        break 'third;
                    } else {
                        Self::build_and_store(&mut addrs, &chars, &s, l, m, r);
                    }
                }
            }
        }

        addrs
    }

    fn build_and_store(
        addrs: &mut Vec<String>,
        chars: &[char],
        s: &String,
        l: usize,
        m: usize,
        r: usize,
    ) {
        let mut first = String::from(&s[..l]);
        let mut second = String::from(&s[l..m]);
        let mut third = String::from(&s[m..r]);
        let mut fourth = String::from(&s[r..]);
        for part in [&first, &second, &third, &fourth] {
            match part.len() {
                1 => continue,
                2 => {
                    if part.chars().nth(0).unwrap() == '0' {
                        return;
                    }
                } // make sure it doent start with a 0
                3 => {
                    if part.chars().nth(0).unwrap() == '0' || part.chars().nth(1).unwrap() == '0' {
                        return;
                    } else {
                        let i = part.parse::<i32>().unwrap();
                        if i > 255 {
                            return;
                        }
                    }
                }
                _ => return,
            }
        }
        let s = format!("{}.{}.{}.{}", first, second, third, fourth);
        addrs.push(s);
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::HashSet;

    #[test]
    fn first() {
        let s = "25525511135".to_string();
        let expected: Vec<String> = ["255.255.11.135", "255.255.111.35"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let ans = Solution::restore_ip_addresses(s);
        let ans = ans.into_iter().collect::<HashSet<String>>();
        let expected = expected.into_iter().collect::<HashSet<String>>();
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let s = "0000".to_string();
        let expected: Vec<String> = ["0.0.0.0"].into_iter().map(|s| s.to_string()).collect();
        let ans = Solution::restore_ip_addresses(s);
        let ans = ans.into_iter().collect::<HashSet<String>>();
        let expected = expected.into_iter().collect::<HashSet<String>>();
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let s = "101023".to_string();
        let expected: Vec<String> = [
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let ans = Solution::restore_ip_addresses(s);
        let ans = ans.into_iter().collect::<HashSet<String>>();
        let expected = expected.into_iter().collect::<HashSet<String>>();
        assert_eq!(ans, expected);
    }
}

use std::cmp::min;
use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut combos = HashSet::new();
        let mut len = s.len();
        if len < 4 || len > 12 {
            return vec![];
        }
        // [0.0.0.0]
        for l in 1..len - 3 {
            for m in l + 1..min(l + 4, len - 2) {
                for r in m + 1..min(m + 4, len - 1) {
                    if let Some(s) = Self::get_str(l, m, r, &s) {
                        println!("{}", s);
                    }
                }
            }
        }

        combos.into_iter().collect()
    }

    fn get_str(l: usize, m: usize, r: usize, s: &String) -> Option<String> {
        if l >= m || m >= r || r >= s.len() {
            return None;
        }
        // check lengths
        // first num is 1,2,
        let first = (l - 0, ..l);
        let second = (m - l, l..m);
        let third = (r - m, m..r);
        let fourth = (s.len() - r, r..s.len());

        for (len, rng) in [first, second, third, fourth].into_iter() {
            if len < 0 || len <= 3 {
                return None;
            } else {
                match s.get(rng.start) {
                    _ => (),
                }
            }
        }
        // format!("{}.{}.{}.{}", &s[..l], &s[l..m], &s[m..r], &s[r..])
        None
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

    // #[test]
    fn second() {
        let s = "0000".to_string();
        let expected: Vec<String> = ["0.0.0.0"].into_iter().map(|s| s.to_string()).collect();
        let ans = Solution::restore_ip_addresses(s);
        let ans = ans.into_iter().collect::<HashSet<String>>();
        let expected = expected.into_iter().collect::<HashSet<String>>();
        assert_eq!(ans, expected);
    }

    // #[test]
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

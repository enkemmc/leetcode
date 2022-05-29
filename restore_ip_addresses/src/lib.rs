struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let len = s.len();
        let chars = s.chars().collect::<Vec<char>>();
        let mut combos = vec![];
        if len < 4 || len > 12 {
            return combos;
        }

        for l in 1..=std::cmp::min(3, len - 2) {
            if !Self::is_valid_starter(&chars, l) {
                continue;
            }
            for m in l + 1..=std::cmp::min(l + 3, len - 1) {
                if !Self::is_valid_starter(&chars, m) {
                    continue;
                }
                for r in m + 1..=std::cmp::min(m + 3, len) {
                    if !Self::is_valid_starter(&chars, l) {
                        continue;
                    }
                    Self::verify(&mut combos, &s, l, m, r);
                }
            }
        }

        combos
    }

    fn is_valid_starter(chars: &[char], i: usize) -> bool {
        if i < chars.len() {
            match chars[i] {
                '0' | '1' | '2' => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn verify(combos: &mut Vec<String>, chars: &String, l: usize, m: usize, r: usize) {
        // check ranges

        // 4th
        let fourth_len = chars.len() - r;
        if fourth_len <= 3 && fourth_len > 0 {
            // 4 - 3 >= 3
            let one = 0..l;
            let two = l..m;
            let three = m..r;
            let four = m..fourth_len;
            if [one, two, three, four].into_iter().any(|r| {
                let s = &chars[r.clone()];
                if s.len() > 1 && &chars[r.start..r.start+1] == "0" {
                    true
                } else {
                    false
                }
            }) {
                return // one of the sequences starts with a 0 and has trailing numbers
            }
            let ip = format!(
                "{}.{}.{}.{}",
                &chars[0..l],
                &chars[l..m],
                &chars[m..r],
                &chars[r..]
            );
            combos.push(ip);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let input = "25525511135".to_owned();
        let expected = vec!["255.255.11.135", "255.255.111.35"];
        let ans = Solution::restore_ip_addresses(input);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let input = "0000".to_owned();
        let expected = vec!["255.255.11.135", "255.255.111.35"];
        let ans = Solution::restore_ip_addresses(input);
        assert_eq!(ans, expected);
    }
}

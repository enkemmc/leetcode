impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut combos = vec![];
        let mut memo = vec![vec![None; s.len()]; s.len()];
        if s.len() > 0 {
            let chars = s.chars().collect::<Vec<char>>();
            Self::helper(&mut combos, &chars, &mut vec![], &mut memo, 0);
        }

        combos.into_iter().collect()
    }

    fn helper(
        combos: &mut Vec<Vec<String>>,
        chars: &[char],
        combo: &mut Vec<String>,
        memo: &mut Vec<Vec<Option<bool>>>,
        start: usize,
    ) {
        if start == chars.len() {
            combos.push(combo.clone());
        } else {
            for end in start + 1..chars.len() {
                // check to see if we've already visited this range
                if let Some(is_valid) = memo[start][end] {
                    if !is_valid {
                        // we know this substr isnt a pallindrome
                        continue;
                    } else {
                        let mut string = String::new();
                        for ch in &chars[start..end] {
                            string.push(*ch);
                        }

                        combo.push(string);
                        Self::helper(combos, chars, combo, memo, end + 1);
                        combos.pop();
                    }
                } else {
                    let mut string = String::new();
                    for ch in &chars[start..end] {
                        string.push(*ch);
                    }

                    if Self::verify(&string) {
                        memo[start][end] = Some(true);
                        combo.push(string);
                        Self::helper(combos, chars, combo, memo, end + 1);
                        combos.pop();
                    } else {
                        println!("{} isnt a pal", string);
                        memo[start][end] = Some(false);
                    }
                }
            }
        }
    }

    // check if its a pallindrome
    fn verify(string: &String) -> bool {
        if string.len() == 1 {
            true
        } else if string.len() == 0 {
            false
        } else {
            let chars: Vec<char> = string.clone().chars().collect();
            let mut l = 0;
            let mut r = chars.len() - 1;
            while l <= r {
                if chars[l] == chars[r] {
                    l += 1;
                    r -= 1;
                } else {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let input = "aab".to_string();
        let ans = Solution::partition(input);
        let expected: Vec<Vec<String>> = vec![
            vec!["a", "a", "b"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["aa", "b"].into_iter().map(|s| s.to_string()).collect(),
        ];
        assert_eq!(expected, ans);
    }

    //#[test]
    fn verify() {
        let pallindrome = vec!["A", "D", "D", "A"].into_iter().collect();
        let ans = Solution::verify(&pallindrome);
        assert_eq!(ans, true);
    }
}

struct Solution;

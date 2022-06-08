use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut combos = HashSet::new();
        let mut max = 0;
        let chars: Vec<String> = s.chars().map(|c| c.to_string()).collect();
        Self::helper(&mut combos, &chars, &mut max);

        max as i32
    }

    fn helper(combos: &mut HashSet<String>, chars: &[String], max: &mut usize) {
        if !chars.is_empty() {
            for i in 0..chars.len() {
                let mut substr = chars[0..i + 1]
                    .iter()
                    .fold(String::new(), |mut new_str, s| {
                        new_str.push_str(s);
                        new_str
                    });
                if !combos.contains(&substr) {
                    combos.insert(substr.clone());
                    Self::helper(combos, &chars[i + 1..], max);
                    combos.remove(&substr);
                }
            }
        }
        if combos.len() > *max {
            *max = combos.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let input = "ababccc".to_string();
        let ans = Solution::max_unique_split(input);
        let expected = 5;
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let input = "wwwzfvedwfvhsww".to_string();
        let expected = 11;
        let ans = Solution::max_unique_split(input);
        assert_eq!(ans, expected);
    }
}

struct Solution;

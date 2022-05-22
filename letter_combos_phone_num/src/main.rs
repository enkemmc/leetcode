use std::collections::HashMap;
fn main() {
    let digits = "23".to_string();
    let _expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let ans = Solution::letter_combinations(digits);
    println!("{:?}", ans);
    // assert_eq!(ans, expected);
}

struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.chars().map(|c| c as u8).collect::<Vec<u8>>();
        let num_to_char = get_map();
        let mut ans = vec![];
        let len = digits.len();
        Self::helper(0, len, &digits, &num_to_char, vec![], &mut ans);
        ans
    }

    fn helper(
        i: usize,
        len: usize,
        digits: &[u8],
        num_to_char: &HashMap<u8, Vec<u8>>,
        combo: Vec<u8>,
        combos: &mut Vec<String>,
    ) {
        if i == len {
            combos.push(String::from_utf8(combo).unwrap());
            return;
        } else {
            let digit = digits[i];
            if let Some(letters) = num_to_char.get(&digit) {
                for letter in letters {
                    let mut combo = combo.clone();
                    combo.push(letter.clone());
                    Self::helper(i + 1, len, digits, num_to_char, combo, combos);
                }
            }
        }
    }
}

fn get_map() -> HashMap<u8, Vec<u8>> {
    let mut num_to_char = HashMap::new();
    num_to_char.insert(b'2', vec![b'a', b'b', b'c']);
    num_to_char.insert(b'3', vec![b'd', b'e', b'f']);
    num_to_char.insert(b'4', vec![b'g', b'h', b'i']);
    num_to_char.insert(b'5', vec![b'j', b'k', b'l']);
    num_to_char.insert(b'6', vec![b'm', b'o', b'n']);
    num_to_char.insert(b'7', vec![b'p', b'q', b'r', b's']);
    num_to_char.insert(b'8', vec![b't', b'u', b'v']);
    num_to_char.insert(b'9', vec![b'w', b'x', b'y', b'z']);

    num_to_char
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut l = 0isize;
        let mut r: isize = -1 + s.len() as isize;
        let bytes = s.bytes().into_iter().collect::<Vec<u8>>();
        let mut combo: (u8, u8);
        while l <= r {
            combo = (bytes[l as usize], bytes[r as usize]);
            if !Self::is_alphanumeric(combo.0) {
                l += 1;
                continue;
            }
            if !Self::is_alphanumeric(combo.1) {
                r -= 1;
                continue;
            }
            // make sure they're both lowercase
            match combo {
                (65..=90, 97..=122) => {
                    combo.1 -= 32; // make r lowercase
                },
                (97..=122, 65..=90) =>  {
                    combo.0 -= 32;
                },
                _ => (),
            }

            // compare
            if combo.0 != combo.1 {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }

    #[inline]
    fn is_alphanumeric(c: u8) -> bool {
        match c {
            65..=90 | 97..=122 | 48..=57 => true,
            _ => false,
        }
    }
}

// a-z 97 - 122
// A-Z 65 - 90
// 0-9 48 - 57
fn main(){
        let s = "A man, a plan, a canal: Panama 0 1 2";
        for c in s.chars() {
            //println!("{} {}", c, Solution::is_alphanumeric(c as u8));
            println!("{} {}", c, c as u8);
        }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn third() {
        let s = "a";
        let ans = Solution::is_palindrome(s.to_string());
        let expected = true;
        assert_eq!(expected, ans);
    }

    #[test]
    fn first() {
        let s = "A man, a plan, a canal: Panama";
        let ans = Solution::is_palindrome(s.to_string());
        let expected = true;
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let s = "0P";
        let ans = Solution::is_palindrome(s.to_string());
        let expected = false;
        assert_eq!(expected, ans);
    }
}

struct Solution;

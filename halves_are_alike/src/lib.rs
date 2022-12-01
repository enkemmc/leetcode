impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut l_count = 0;
        let mut r_count = 0;

        let mid = s.len() / 2;

        let (l, r) = s.split_at(mid);
        for c in l.chars() {
            if Self::is_vowel(c) {
                l_count += 1;
            }
        }

        for c in r.chars() {
            if Self::is_vowel(c) {
                r_count += 1;
            }
            if r_count > l_count {
                return false;
            }
        }

        l_count == r_count
    }

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' |
            'A' | 'E' | 'I' | 'O' | 'U'  => true,
            _ => false
        }
    }
}

enum Shapes {
    Circle,
    Square
}

impl Shape {
    fn do_thing(&self) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Solution;

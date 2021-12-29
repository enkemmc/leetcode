struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut last = 0;
        for c in s.chars() {
            if last >= get_value(c) || last == 0 {
                sum += last;
                last = get_value(c);
            } else {
                sum += get_value(c) - last;
                last = 0;
            }

        }
        sum + last
    }
}

fn get_value(ch: char) -> i32 {
    match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let s = "III".to_string();
        let expected = 3;
        let ans = Solution::roman_to_int(s);
        assert_eq!(expected, ans);
    }
    #[test]
    fn test2() {
        let s = "LVIII".to_string();
        let expected = 58;
        let ans = Solution::roman_to_int(s);
        assert_eq!(expected, ans);
    }

    #[test]
    fn test3() {
        let s = "MCMXCIV".to_string();
        let expected = 1994;
        let ans = Solution::roman_to_int(s);
        assert_eq!(expected, ans);
    }

    #[test]
    fn test4() {
        let s = "DCXXI".to_string();
        let expected = 621;
        let ans = Solution::roman_to_int(s);
        assert_eq!(expected, ans);
    }
}

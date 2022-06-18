impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<ParenType> = vec![];
        let mut chars = s.chars();
        let first: ParenType = chars.next().unwrap().into();
        if !first.is_open() {
            return false;
        }
        stack.push(first);

        for ch in chars {
            let paren_type: ParenType = ch.into();
            if paren_type.is_open() {
                stack.push(paren_type);
            } else {
                if let Some(end) = stack.pop() {
                    if !paren_type.is_pair(&end) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

#[derive(Eq, PartialEq)]
enum ParenType {
    CurlyOpen,
    RoundOpen,
    SquareOpen,
    CurlyClosed,
    RoundClosed,
    SquareClosed,
}

impl From<char> for ParenType {
    fn from(c: char) -> Self {
        match c {
            '(' => Self::RoundOpen,
            ')' => Self::RoundClosed,
            '[' => Self::SquareOpen,
            ']' => Self::SquareClosed,
            '{' => Self::CurlyOpen,
            '}' => Self::CurlyClosed,
            _ => unreachable!(),
        }
    }
}

impl ParenType {
    fn is_open(&self) -> bool {
        match self {
            ParenType::CurlyOpen | ParenType::SquareOpen | ParenType::RoundOpen => true,
            _ => false,
        }
    }

    fn is_pair(&self, cmp: &ParenType) -> bool {
        match *cmp {
            ParenType::RoundOpen => *self == ParenType::RoundClosed,
            ParenType::RoundClosed => *self == ParenType::RoundOpen,
            ParenType::SquareOpen => *self == ParenType::SquareClosed,
            ParenType::SquareClosed => *self == ParenType::SquareOpen,
            ParenType::CurlyOpen => *self == ParenType::CurlyClosed,
            ParenType::CurlyClosed => *self == ParenType::CurlyOpen,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let input = "()".to_string();
        let ans = Solution::is_valid(input);
        assert_eq!(true, ans);
    }

    #[test]
    fn second() {
        let input = "[()]".to_string();
        let ans = Solution::is_valid(input);
        assert_eq!(true, ans);
    }

    #[test]
    fn third() {
        let input = "(])[".to_string();
        let ans = Solution::is_valid(input);
        assert_eq!(false, ans);
    }

    #[test]
    fn fourth() {
        let input = "([)]".to_string();
        let ans = Solution::is_valid(input);
        assert_eq!(false, ans);
    }

    #[test]
    fn fifth() {
        let input = "[()()]".to_string();
        let ans = Solution::is_valid(input);
        assert_eq!(true, ans);
    }
}

struct Solution;

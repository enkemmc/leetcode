const OFFSET: u8 = 48;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut stack = vec![];
        let mut remainder = 0;
        let mut ar = a.bytes().rev().peekable();
        let mut br = b.bytes().rev().peekable();

        loop {
            match ar.next().unwrap_or(OFFSET) + br.next().unwrap_or(OFFSET) + remainder - 2 *  OFFSET {
                0 => {
                    stack.push(OFFSET);
                    remainder = 0;
                }
                1 => {
                    stack.push(1 + OFFSET);
                    remainder = 0;
                }
                2 => {
                    stack.push(OFFSET);
                    remainder = 1;
                },
                3 => {
                    stack.push(1 + OFFSET);
                    remainder = 1;
                },
                _ => {
                    unreachable!()
                }
            }
            if ar.peek().is_none() && br.peek().is_none() {
                break;
            }
        }

        if remainder == 1 {
            stack.push(1 + OFFSET);
        }
        stack.reverse();
        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn first() {
        let a = "11".into();
        let b = "1".into();
        let expected = "100".to_string();
        let ans = Solution::add_binary(a, b);
        assert_eq!(ans, expected);
    }
}

struct Solution;

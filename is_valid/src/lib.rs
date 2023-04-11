impl Solution {
    pub fn is_valid(s: String) -> bool {
        //[(, {, [ ]
        let mut stack = vec![];
        let empty = [0,0,0];

        for s in s.chars() {
            match s {
                '(' => {
                    stack.push([1,0,0]);
                },
                ')' => {
                    if let Some(mut state) = stack.pop() {
                        if state[0] == 0 {
                            return false;
                        } else {
                            state[0] -= 1;
                            if state != empty {
                                stack.push(state);
                            }
                        }
                    } else {
                        return false;
                    }
                },
                '{' => {
                    stack.push([0,1,0]);
                },
                '}' => {
                    if let Some(mut state) = stack.pop() {
                        if state[1] == 0 {
                            return false;
                        } else {
                            state[1] -= 1;
                            if state != empty {
                                stack.push(state);
                            }
                        }
                    } else {
                        return false;
                    }
                },
                '[' => {
                    stack.push([0,0,1]);
                },
                ']' => {
                    if let Some(mut state) = stack.pop() {
                        if state[2] == 0 {
                            return false;
                        } else {
                            state[2] -= 1;
                            if state != empty {
                                stack.push(state);
                            }
                        }
                    } else {
                        return false;
                    }
                },
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let s = "()".to_string();
        let ans = Solution::is_valid(s);
        let expected = true;
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let s = "()[]{}".to_string();
        let ans = Solution::is_valid(s);
        let expected = true;
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let s = "([".to_string();
        let ans = Solution::is_valid(s);
        let expected = false;
        assert_eq!(ans, expected);
    }

}

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            match token.as_str() {
                "/" | "+" | "-" | "*" => {
                    // last num in stack is right
                    // second to last in stack is left
                    let (right, left) = (stack.pop().unwrap(), stack.pop().unwrap());
                    let ans = match token.as_str() {
                        "/" => left / right,
                        "*" => left * right,
                        "+" => left + right,
                        "-" => left - right,
                        _ => unreachable!()
                    };
                    //println!("{} {} {} = {}", left, token.as_str(), right, ans);
                    stack.push(ans);
                },
                num => {
                    // this means we're dealing with a number
                    stack.push(num.parse::<i32>().unwrap());
                }
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        // 2 + 1 + 3
        let tokens = 
           vec!["2","1","+","3","*"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let ans = Solution::eval_rpn(tokens);
        let expected = 9;
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        // 2 + 1 + 3
        let tokens = 
            vec!["4","13","5","/","+"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let ans = Solution::eval_rpn(tokens);
        let expected = 6;
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        // 2 + 1 + 3
        let tokens = 
            vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let ans = Solution::eval_rpn(tokens);
        let expected = 22;
        assert_eq!(ans, expected);
    }
}

struct Solution;


impl Solution {
    pub fn remove_stars(s: String) -> String {
        //let mut stack = Vec::with_capacity(s.len());
        let mut ans = String::with_capacity(s.len());
        for c in s.chars() {
            if c == '*' {
                ans.pop();
            } else {
                ans.push(c);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct Solution;

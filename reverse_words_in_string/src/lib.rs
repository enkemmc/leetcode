const SPACE: &str = "";

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let spl = s.split(' ');
        let mut stack = vec![];
        for st in spl {
            if st != SPACE {
                let n = st.trim();
                stack.push(n);
            }
        }

        stack.reverse();
        stack.join(" ")
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn first() {
        let s = vec!["pears", "            jungle ", "  apple "].into_iter().map(|s| s.to_owned()).collect();
        let ans = Solution::reverse_words(s);
        println!("{:?}", ans);
        let result = 4;
        assert_eq!(result, 4);
    }
}

struct Solution;

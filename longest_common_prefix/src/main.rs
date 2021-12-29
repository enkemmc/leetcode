struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let mut i = 0;

        while strs.iter().all(|s| s.get(i..i+1) == strs[0].get(i..i+1) && s.get(i..i+1).is_some()){
            i += 1;
        }

        String::from(strs[0].get(0..i).unwrap())
    }
}

fn main(){
    let s = "asdf".to_string();
    let o = String::from(s.get(0..1).unwrap());
    println!("{}", o) ;
}

mod test {
    use crate::Solution;

    #[test]
    fn test1(){
        let strs = ["flower","flow","flight"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expected = "fl".to_string();
        let ans = Solution::longest_common_prefix(strs);
        assert_eq!(expected, ans);
    }

    #[test]
    fn test2(){
        let strs = ["dog","racecar","car"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expected = "".to_string();
        let ans = Solution::longest_common_prefix(strs);
        assert_eq!(expected, ans);
    }
}
fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let nu = turned_on as u32;
        let mut result = Vec::new();
        for i in 0i32..12 {
            for j in 0i32..60 {
                if (i.count_ones() + j.count_ones()) == nu {
                    result.push(format!("{}:{:02}", i, j));
                }
            }
        }
        result
    }
}

mod test {
    use super::*;

    #[test]
    fn test_one() {
        let turnedOn = 1;
        let expected = vec![
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let ans = Solution::read_binary_watch(1);
        assert_eq!(ans, expected);
    }
}

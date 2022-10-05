// [0, 1,3,4,0,2,6,8]
// ex1
// [1,2,3,4,6,8] -> {1,3,4]
// ex2
// [0,0,1,2,3,4,6,8]
// [1,2,3,4,6,8]
// [3,4,6,8]
// [6,8]

impl Solution {
pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn firsdt() {
        let original = vec![1,3,4];
        let mut changed = original.clone();
        let mut doubled = Vec::with_capacity(changed.len());
        for num in &changed[..changed.len()] {
            doubled.push(*num * 2);
        }
        changed.append(&mut doubled);
        println!("{:?}", changed);
        let ans = Solution::find_original_array(changed);
        assert_eq!(ans, original);
    }
}

struct Solution;

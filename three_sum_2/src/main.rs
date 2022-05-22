use std::collections::HashMap;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let tar = 9;
    let ans = Solution::solve(nums, tar);
    println!("{:?}", ans);
}



struct Solution;
impl Solution {
    fn solve(nums: Vec<i32>, tar: i32) -> Vec<i32>{
        let mut ans = vec![];
        let mut numToIndex = HashMap::new();
        let mut compliment;

        for (index, &num) in nums.iter().enumerate() {
            compliment = tar - num;
            if numToIndex.contains_key(&compliment) {
                ans.push(*numToIndex.get(&compliment).unwrap() as i32);
                ans.push(index as i32);
            } else {
                numToIndex.insert(num, index);
            }
           
        }

        ans
    }
}
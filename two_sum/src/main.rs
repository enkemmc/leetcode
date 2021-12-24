use std::collections::HashMap;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let ans = Solution::two_sum(nums,target);
    println!("{:?}", ans);
}

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliment_index_map = HashMap::new();

        for (i, num) in nums.into_iter().enumerate() {
            let compliment = target - num;
            if compliment_index_map.contains_key(&compliment){
                let comp_index = compliment_index_map.get(&compliment).unwrap();
                return vec![i as i32, compliment];
            } else {
                compliment_index_map.insert(num, i);
            }
        }

        unreachable!("asdf!");
    }
}
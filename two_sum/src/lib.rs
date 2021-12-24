use std::collections::HashMap;
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

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_to_test_ok(){
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let ans = HashSet::from([1, 2]);

        let output = Solution::two_sum(nums, target);

        let matches = output.iter().all(|i| ans.contains(i));
        assert!(matches);
    }

    #[test]
    #[should_panic]
    fn test_to_panic(){
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let ans = HashSet::from([1, 3]);
        let output = Solution::two_sum(nums, target);
        let matches = output.iter().all(|i| ans.contains(i));
        assert!(matches);
    }
}
use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut triplets = HashSet::new();
        let mut sum_to_pair = HashMap::new();

        if nums.len() < 3 {
            vec![]
        } else {
            nums.sort();
        
            for i in 0..nums.len() - 2 {
                let target = 0 - nums[i];
                if i == 0 {
                    let mut sum;
                    for j in i+1..nums.len() - 1 {
                        for k in j+1..nums.len() {
                            sum = nums[j] + nums[k];
                            sum_to_pair.entry(sum).or_insert(HashSet::new()).insert((j, k)); 
                            if sum == target {
                                triplets.insert(vec![nums[i], nums[j], nums[k]]);
                            }
                        }
                    }
                    //let mut j = i+1;
                    //let mut k = nums.len() - 1;

                    //let mut sum;
                    //while j < k {
                    //    sum = nums[j] + nums[k];
                    //    if i == 0 {
                    //        sum_to_pair.entry(sum).or_insert(HashSet::new()).insert((j, k));
                    //    }

                    //    if sum < target {
                    //        j += 1;
                    //    } else if sum > target {
                    //        k -= 1;
                    //    } else {
                    //        triplets.insert(vec![nums[i], nums[j], nums[k]]);
                    //        j += 1;
                    //        k -=1;
                    //    }
                    //}
                } else {
                    if let Some(pairs) = sum_to_pair.get(&target) {
                        for &(j,  k) in pairs {
                            if i < j {
                                triplets.insert(vec![nums[i], nums[j], nums[k]]);
                            }
                        }
                    }
                }
            }
            
            triplets.into_iter().collect::<Vec<Vec<i32>>>()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let nums = vec![-1,0,1,2,-1,-4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let ans = Solution::three_sum(nums);

        assert_eq!(expected, ans);
    }
}

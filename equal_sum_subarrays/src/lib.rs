use std::collections::HashMap;


impl Solution {
    pub fn equal_sum_subarray(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut total = 0;
        let mut sum_to_indices = HashMap::new();
        for i in 0..arr.len() {
            total += arr[i];
            sum_to_indices.entry(total).or_insert_with(Vec::new).push(i as usize);
        }

        if total % 2 != 0 {
            return vec![];
        }
        let compliment = total / 2;
        if let Some(indices) = sum_to_indices.get(&compliment) {
            for &i in indices {
                return vec![arr[..i+1].to_vec(), arr[i+1..].to_vec()];
            }
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn first() {
        let input = vec![1,3,0,4];
        let ans = Solution::equal_sum_subarray(input);
        let expected = vec![vec![1,3], vec![0,4]];
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let input = vec![0,4];
        let ans = Solution::equal_sum_subarray(input);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(ans, expected);
    }

}

struct Solution;

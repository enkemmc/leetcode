use std::collections::HashMap;

type Pair = [i32;2];


impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let MOD: i32 = 1000000007;
        let num_to_factors = Self::build_map(&arr);
        let num_to_combos = Self::build_combos(&arr);
        let mut count = 0;

        for (num, pairs) in &num_to_factors {
            if let Some(combos) = num_to_factors.get(num) {
                count += 1;
                if !combos.is_empty() {
                    for combo in combos {
                        let left = combo[0];
                        let right = combo[1];
                        let left_combos = num_to_combos.get(&left).unwrap();
                        let right_combos = num_to_combos.get(&right).unwrap();
                        let ans = left_combos * right_combos;
                        if left == right {
                            count += ans;
                        } else {
                            count += 2 * ans;
                        }
                    }
                }
            }

        }

        unimplemented!()
    }

    fn build_map(arr: &Vec<i32>) -> HashMap<i32, Vec<Pair>> {
        HashMap::new()
    }

    fn build_combos(arr: &Vec<i32>) -> HashMap<i32, i32> {
        HashMap::new()
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn first() {
        let arr = vec![2, 4];
        let expected = 3;
        let ans = Solution::num_factored_binary_trees(arr);
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let arr = vec![2, 4, 5, 10];
        let expected = 7;
        let ans = Solution::num_factored_binary_trees(arr);
        assert_eq!(expected, ans);
    }


}

struct Solution;

use std::cmp::max;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut cache = vec![0;len];
        cache[0] = 1;
        let mut ans = 1;

        for i in 1..len {
            let num = nums[i];
            // can we append num to any subsequence to the left to create a longer
            // subsequence?
            //      yes?  great.  do that and store that length in the cache
            //      no? ok. insert 1 as the longest
            let mut longest = 0;

            for (li, &length) in cache[..i].iter().enumerate() {
                let prev_num = nums[li];
                if prev_num < num {
                    longest = max(longest, length);
                }
            }
            // longest is now the length of the longest subseq that we could legally append num to

            cache[i] = longest + 1;
            ans = max(ans, cache[i]);
        }

        ans
    }
}



#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let nums = vec![10,9,2,5,3,7,101,18];
        let expected = 4;
        println!("input:\r\n{:?}", nums);
        let ans = Solution::length_of_lis(nums);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let nums = vec![0,1,0,3,2,3];
        let expected = 4;
        let ans = Solution::length_of_lis(nums);
        assert_eq!(ans, expected);
    }

}

struct Solution;

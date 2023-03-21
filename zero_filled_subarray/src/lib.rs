impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut total_count = 0;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] != 0 {
                i += 1;
                continue;
            } else {
                let mut j = 0;
                while i+j < nums.len() && nums[i+j] == 0 {
                    j += 1;
                }
                let count = do_thing(j);
                total_count += count;
                i += j;
            }
        }
        total_count
    }
}

#[inline]
fn do_thing(num: usize) -> i64 {
    // i is > 1
    let mut i = 0;
    for j in 1..=num {
        i = i + j;
    }
    i as i64
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct Solution;

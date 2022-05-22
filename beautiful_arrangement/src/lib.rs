// nums = nums that are still avialable
// perm = nums that have been used
struct Solution;
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut sum = 0;
        let nums = (1..=n).collect::<Vec<i32>>();
        Self::get_perms(nums, vec![], &mut sum);

        sum
    }

    fn get_perms(nums: Vec<i32>, perm: Vec<i32>, sum: &mut i32) {
        if nums.len() == 0 {
            println!("{:?}", perm);
            *sum += 1;
        } else {
            for i in 0..nums.len() {
                // check to make sure that this num will produce a "beautiful arrangement"
                if nums[i] % (1 + perm.len()) as i32 == 0 || (1 + perm.len()) as i32 % nums[i] == 0
                {
                    let mut perm = perm.clone();
                    let mut nums = nums.clone();
                    let num = nums.remove(i);
                    perm.push(num);
                    Self::get_perms(nums, perm, sum);
                }
                // no else condition here because if one num in the perm is not beautiful, then we're done
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    fn first() {
        let input = 2;
        let expected = 2;
        let ans = Solution::count_arrangement(input);
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let input = 3;
        let expected = 3;
        let ans = Solution::count_arrangement(input);
        assert_eq!(expected, ans);
    }
}

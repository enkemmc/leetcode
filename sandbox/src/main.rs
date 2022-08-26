fn main() {
    let mut v = vec![5,2,6,1];
    let ans = Solution::count_smaller(v);
    println!("{:?}",ans);
}


struct Solution;
impl Solution {
    pub fn count_smaller(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut output = vec![0;len];
        let mut count = 0;
        for i in (0..len).rev() {
            nums[i+1..].sort_unstable();
            let mut n = i + 1;
            while n < len && nums[i] > nums[n] {
                count += 1;
                n += 1;
            }
            output[i] = count;
            count = 0;
        }
        output
    }
}

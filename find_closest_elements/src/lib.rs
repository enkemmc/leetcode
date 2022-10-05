// [1,1,1,7,7,7] k = 4, x = 3
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if k == 1 {
            let mut closest = arr[0];
            let mut iter = arr[1..].into_iter();
            while let Some(&num) = iter.next() {
                if Self::is_closer(num, closest, x) {
                    println!("{} : {}", num, closest);
                    closest = num;
                } else {
                    if num != closest {
                    println!("{} : {} exiting", num, closest);
                        break;
                    }
                }
            }
            vec![closest]
        } else {
            let mut l = 0;
            let mut r = k as usize - 1;
            while r < arr.len() - 1 && !Self::is_closer(arr[l+ 1], arr[r + 1], x) {
                l += 1;
                r += 1;
            }
            arr[l..=r].to_vec()
        }
    }

    // is a closer to x than b?
    fn is_closer(a: i32, b: i32, x: i32) -> bool {
        (a - x) < (b - x) || (a - x) == (b - x) && a < b
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    fn first() {
        let input = vec![1,2,3,4,5];
        let k = 4;
        let x = 3;
        let ans = Solution::find_closest_elements(input, k, x);
        let expected = vec![1,2,3,4];
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let input = vec![1,1,1,10,10,10];
        let k = 1;
        let x = 9;
        let ans = Solution::find_closest_elements(input, k, x);
        let expected = vec![10];
        assert_eq!(expected, ans);
    }

    fn third(){
        let input = vec![1,2];
        let k = 1;
        let x = 1;
        let ans = Solution::find_closest_elements(input, k, x);
        let expected = vec![1];
        assert_eq!(expected, ans);
    }
}

struct Solution;

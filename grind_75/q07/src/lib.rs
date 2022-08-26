impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            if nums[l] == target {
                return l as i32;
            } else if nums[r] == target {
                return r as i32;
            } else {
                let mid = r - l;
                if nums[mid] < target {
                    l = mid + 1;
                } else if nums[mid] > target {
                    if mid <= 1 {
                        break;
                    }
                    r = mid - 1;
                } else {
                    return mid as i32;
                }
            }
        }
        -1
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Solution;

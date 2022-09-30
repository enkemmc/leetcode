use std::collections::BinaryHeap;
use std::cmp::Reverse;


impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut iter = arr.iter().enumerate();
        let mut closest_index_to_val = (0, x.abs_diff(*iter.next().unwrap().1));

        for (index, num) in iter {
            if x.abs_diff(*num) < closest_index_to_val.1 {
                closest_index_to_val = (index, x.abs_diff(*num));
            }
        }
        let i = closest_index_to_val.0;

        // add k to the left and right
        let mut count = 0;
        let mut index = i;
        heap.push(Reverse(arr[i]));
        while index > 0 && count < k {
            index -= 1;
            heap.push(Reverse(arr[index]));
            count += 1;
        }

        index = i;
        count = 0;

        while index < arr.len() - 1 && count < k {
            index += 1;
            heap.push(Reverse(arr[index]));
            count += 1;
        }
        

        let mut ans = vec![];
        while let Some(Reverse(num)) = heap.pop() {
            ans.push(num);
            if ans.len() == k as usize {
                break;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
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
}

struct Solution;

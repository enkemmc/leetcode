use std::cmp::{max, min};

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        if arr[start as usize] == 0 {
            return true;
        }
        let len = arr.len();
        let mut visited = vec![false; len];
        let mut stack = vec![start as usize];
        // O(n) so far

        let mut l = 0usize;
        let mut r = 0usize;
        while let Some(i) = stack.pop() {
            if visited[i] {
                continue;
            } else {
                visited[i] = true;
                if (arr[i] as usize) <= i {
                    l = i - arr[i] as usize;
                    if arr[l] == 0 {
                        return true;
                    } else {
                        stack.push(l);
                    }
                }

                if (i + arr[i] as usize) <= len - 1 {
                    r = i + arr[i] as usize;
                    if arr[r] == 0 {
                        return true;
                    } else {
                        stack.push(r);
                    }
                }
            }
        }
        false
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

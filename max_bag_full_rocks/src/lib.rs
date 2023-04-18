impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        // sort by least # of rocks required to reach capacity?
        let mut rocks_needed = Vec::with_capacity(rocks.len());
        for (c, r) in capacity.into_iter().zip(rocks) {
            rocks_needed.push(c - r);
        }
        rocks_needed.sort_unstable();
        let mut ans = 0;

        for bag in rocks_needed {
            if bag <= additional_rocks {
                ans += 1;
                additional_rocks -= bag;
            } else {
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
        let capacity = vec![2,3,4,5];
        let rocks = vec![1,2,4,4];
        let additional_rocks = 2;
        let expected = 3;
        let ans = Solution::maximum_bags(capacity, rocks, additional_rocks);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let capacity = vec![10,2,2];
        let rocks = vec![2,2,0];
        let additional_rocks = 100;
        let expected = 3;
        let ans = Solution::maximum_bags(capacity, rocks, additional_rocks);
        assert_eq!(ans, expected);
    }

}

struct Solution;

struct Solution;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut memo = grid.clone();
        for row in memo.iter_mut() {
            for cell in row.iter_mut() {
                *cell = i32::MAX;
            }
        }
        let mut min_sum = i32::MAX;
        let end_pos = (grid[0].len() - 1, grid.len() - 1);
        grid.iter().for_each(|r| println!("{:?}", r));
        println!("end pos: {:?}", end_pos);
        Self::helper(&mut min_sum, &grid, (0, 0), end_pos, 0, &mut memo);
        min_sum
    }

    fn helper(
        min_sum: &mut i32,
        grid: &Vec<Vec<i32>>,
        pos: (usize, usize), // (x, y)
        end_pos: (usize, usize),
        mut sum: i32,
        memo: &mut Vec<Vec<i32>>,
    ) {
        if pos.0 == end_pos.0 && pos.1 == end_pos.1 {
            sum += Self::get_value(pos, grid);
            if sum < *min_sum {
                *min_sum = sum;
            }
        } else {
            if Self::is_pos_valid(pos, end_pos, &min_sum, sum, memo) {
                let cell_value = Self::get_value(pos, grid);
                // println!("{:?} is valid", pos);
                // right
                let mut right_pos = pos.clone();
                right_pos.0 += 1;
                Self::helper(min_sum, grid, right_pos, end_pos, sum + cell_value, memo);
                // down
                let mut left_pos = pos.clone();
                left_pos.1 += 1;
                Self::helper(min_sum, grid, left_pos, end_pos, sum + cell_value, memo);
            }
        }
    }

    fn get_value(pos: (usize, usize), grid: &Vec<Vec<i32>>) -> i32 {
        grid[pos.1][pos.0]
    }

    fn is_pos_valid(
        pos: (usize, usize),
        end_pos: (usize, usize),
        min_sum: &i32,
        sum: i32,
        memo: &mut Vec<Vec<i32>>,
    ) -> bool {
        // if its out of bounds
        // if its sum is bigger than min_sum
        // if we've been to this position before and our sum was smaller
        if pos.0 > end_pos.0 || pos.1 > end_pos.1 {
            return false;
        }

        if sum < Self::get_value(pos, memo) {
            memo[pos.1][pos.0] = sum;
        } else {
            // we've been to this position before and our sum was smaller
            return false;
        }

        if min_sum < &sum {
            return false;
        } else {
            return pos.0 <= end_pos.0 && pos.1 <= end_pos.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let expected = 7;
        let ans = Solution::min_path_sum(grid);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = 12;
        let ans = Solution::min_path_sum(grid);
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let grid = vec![
            vec![3, 8, 6, 0, 5, 9, 9, 6, 3, 4, 0, 5, 7, 3, 9, 3],
            vec![0, 9, 2, 5, 5, 4, 9, 1, 4, 6, 9, 5, 6, 7, 3, 2],
            vec![8, 2, 2, 3, 3, 3, 1, 6, 9, 1, 1, 6, 6, 2, 1, 9],
            vec![1, 3, 6, 9, 9, 5, 0, 3, 4, 9, 1, 0, 9, 6, 2, 7],
            vec![8, 6, 2, 2, 1, 3, 0, 0, 7, 2, 7, 5, 4, 8, 4, 8],
            vec![4, 1, 9, 5, 8, 9, 9, 2, 0, 2, 5, 1, 8, 7, 0, 9],
            vec![6, 2, 1, 7, 8, 1, 8, 5, 5, 7, 0, 2, 5, 7, 2, 1],
            vec![8, 1, 7, 6, 2, 8, 1, 2, 2, 6, 4, 0, 5, 4, 1, 3],
            vec![9, 2, 1, 7, 6, 1, 4, 3, 8, 6, 5, 5, 3, 9, 7, 3],
            vec![0, 6, 0, 2, 4, 3, 7, 6, 1, 3, 8, 6, 9, 0, 0, 8],
            vec![4, 3, 7, 2, 4, 3, 6, 4, 0, 3, 9, 5, 3, 6, 9, 3],
            vec![2, 1, 8, 8, 4, 5, 6, 5, 8, 7, 3, 7, 7, 5, 8, 3],
            vec![0, 7, 6, 6, 1, 2, 0, 3, 5, 0, 8, 0, 8, 7, 4, 3],
            vec![0, 4, 3, 4, 9, 0, 1, 9, 7, 7, 8, 6, 4, 6, 9, 5],
            vec![6, 5, 1, 9, 9, 2, 2, 7, 4, 2, 7, 2, 2, 3, 7, 2],
            vec![7, 1, 9, 6, 1, 2, 7, 0, 9, 6, 6, 4, 4, 5, 1, 0],
            vec![3, 4, 9, 2, 8, 3, 1, 2, 6, 9, 7, 0, 2, 4, 2, 0],
            vec![5, 1, 8, 8, 4, 6, 8, 5, 2, 4, 1, 6, 2, 2, 9, 7],
        ];
        let expected = 7;
        let ans = Solution::min_path_sum(grid);
        assert_eq!(ans, expected);
    }
}

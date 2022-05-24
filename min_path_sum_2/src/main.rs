fn main() {
    println!("Hello world!");
}

struct Solution;
impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;

        let m_len = grid[0].len(); // width
        let n_len = grid.len(); // height

        // update top row
        for i in 1..m_len {
            grid[0][i] += grid[0][i-1];
        }

        // update left column
        for i in 1..n_len {
            grid[i][0] += grid[i-1][0];
        }

        for ni in 1..n_len {
            for mi in 1..m_len {
                grid[ni][mi] = min(grid[ni - 1][mi], grid[ni][mi-1]) + grid[ni][mi];
            }
        }

        grid[n_len - 1][m_len - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let i = 0;
        let expected = 7;
        let ans = Solution::min_path_sum(grid);
        assert_eq!(expected, ans);
    }
}

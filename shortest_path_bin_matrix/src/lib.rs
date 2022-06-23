use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut vd = VecDeque::new();
        if grid[0][0] == 0 {
            vd.push_front(((0, 0), 1)); // (position, sum)
        }

        let mut shortest = vec![vec![std::i32::MAX; grid.len()]; grid.len()];
        if grid[0][0] == 0 {
            shortest[0][0] = 1;
        }

        let dirs = vec![
            (-1, 0),  // L
            (-1, -1), // UL
            (0, -1),  // U
            (1, -1),  // UR
            (1, 0),   // R
            (1, 1),
            (0, 1),
            (-1, 1),
        ];

        while let Some(state) = vd.pop_front() {
            let (pos, len) = state;
            for dir in &dirs {
                let new_pos = ((pos.0 as isize + dir.0), (pos.1 as isize + dir.1));
                if new_pos.0 < grid.len() as isize
                    && new_pos.0 >= 0
                    && new_pos.1 < grid.len() as isize
                    && new_pos.1 >= 0
                    && grid[new_pos.1 as usize][new_pos.0 as usize] == 0
                    && shortest[new_pos.1 as usize][new_pos.0 as usize] > len + 1
                {
                    // path fits conditions and is a shorter path than we've found
                    shortest[new_pos.1 as usize][new_pos.0 as usize] = len + 1;
                    vd.push_back(((new_pos.0 as usize, new_pos.1 as usize), len + 1));
                }
            }
        }

        if shortest[grid.len() - 1][grid.len() - 1] == std::i32::MAX {
            -1
        } else {
            shortest[grid.len() - 1][grid.len() - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let ans = Solution::shortest_path_binary_matrix(grid);
        let expected = 2;
        assert_eq!(expected, ans);
    }

    #[test]
    fn second() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let ans = Solution::shortest_path_binary_matrix(grid);
        let expected = -1;
        assert_eq!(expected, ans);
    }

    #[test]
    fn third() {
        let grid = vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 1, 0]];
        let ans = Solution::shortest_path_binary_matrix(grid);
        let expected = 3;
        assert_eq!(expected, ans);
    }

    #[test]
    fn fourth() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let ans = Solution::shortest_path_binary_matrix(grid);
        let expected = 4;
        assert_eq!(expected, ans);
    }
}

struct Solution;

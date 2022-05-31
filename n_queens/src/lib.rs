struct Solution;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut nums: Vec<i32> = (0..n).into_iter().collect();
        let mut perms = vec![];
        let mut valid_positions = Self::get_board(n); // 0 means its safe
        Self::helper(&mut nums, &mut perms, 0, &mut valid_positions);
        // print permutations
        for perm in &perms {
            println!("{:?}", perm);
        }
        perms.into_iter().map(Self::board_to_string).collect()
    }

    fn get_board(n: i32) -> Vec<Vec<usize>> {
        (0..n)
            .into_iter()
            .map(|_| (0..n).into_iter().map(|_| 0).collect::<Vec<usize>>())
            .collect()
    }

    fn board_to_string(perm: Vec<i32>) -> Vec<String> {
        vec![]
    }

    fn helper(
        nums: &mut Vec<i32>,
        perms: &mut Vec<Vec<i32>>,
        k: usize,
        valid_positions: &mut Vec<Vec<usize>>,
    ) {
        // recursive call for generating all perms
        if k == nums.len() {
            perms.push(nums.clone());
        } else {
            for i in k..nums.len() {
                // see if its safe to place queen
                if Self::is_valid(&valid_positions, nums, i) {
                    nums.swap(i, k);
                    Self::mark_lines(valid_positions, i, nums[i] as usize, false);
                    Self::helper(nums, perms, k + 1, valid_positions);
                    nums.swap(i, k);
                    Self::mark_lines(valid_positions, i, nums[i] as usize, true);
                }
            }
        }
    }

    fn mark_lines(
        valid_positions: &mut Vec<Vec<usize>>,
        row_i: usize,
        col_i: usize,
        validity: bool,
    ) {
        let len = valid_positions.len();
        // row
        for i in 0..len {
            if i != row_i {
                // dont mark actual queen position yet
                if validity {
                    valid_positions[row_i][i] -= 1;
                } else {
                    valid_positions[row_i][i] += 1;
                }
            }
        }
        // col
        for i in 0..len {
            if i != col_i {
                // dont mark actual queen position yet
                if validity {
                    valid_positions[i][col_i] -= 1;
                } else {
                    valid_positions[i][col_i] += 1;
                }
            }
        }

        let mut x = col_i;
        let mut y = row_i;
        // diagonals

        // top left to bot right
        while x > 0 && y > 0 {
            x -= 1;
            y -= 1;
        }

        while x < len && y < len {
            Self::set_validity(valid_positions, y, x, validity); // mark queen position here
            x += 1;
            y += 1;
        }

        x = col_i;
        y = row_i;
        // top right to bot left
        while x < len && y > 0 {
            x += 1;
            y -= 1;
        }

        while x > 0 && y < len {
            if !(x == col_i && y == row_i) {
                Self::set_validity(valid_positions, y, x, validity); // dont mark queen position here
            }
            x -= 1;
            y += 1;
        }
    }

    fn is_valid(valid_positions: &Vec<Vec<usize>>, nums: &[i32], i: usize) -> bool {
        // i represents row, nums[i] represents column on board
        valid_positions[i][nums[i] as usize] == 0
    }

    fn set_validity(
        valid_positions: &mut Vec<Vec<usize>>,
        row_i: usize,
        col_i: usize,
        validity: bool, // true makes it safer, false makes it less safe
    ) {
        if validity {
            valid_positions[row_i][col_i] -= 1; // if true, subtract.  0 means square is safe again
        } else {
            valid_positions[row_i][col_i] += 1; // if false, add.  the bigger the number, the less safe the square
        }
    }

    fn update_positions() {}
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let n = 4;
        let states = Solution::solve_n_queens(n);
        assert_eq!(4, 4);
    }
}

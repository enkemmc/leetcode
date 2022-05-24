struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let height = board.len();
        let width = board[0].len();
        let mut memo = Self::get_board(height, width);
        let mut word = word.chars().collect::<Vec<char>>();

        for j in 0..height {
            for i in 0..width {
                if Self::helper(&word, &board, i, j, &mut 0, &mut memo) {
                    return true;
                }
            }
        }

        false
    }

    fn helper(
        word: &[char],
        board: &Vec<Vec<char>>,
        x: usize,
        y: usize,
        index: &mut usize,
        memo: &mut Vec<Vec<bool>>,
    ) -> bool {
        if memo[y][x] {
            return false;
        }

        if word[*index] == board[y][x] {
            memo[y][x] = true;

            // check if we're done
            if *index == word.len() - 1 {
                return true;
            }

            *index += 1;
            // right
            if x + 1 < board[0].len() {
                if Self::helper(word, board, x + 1, y, index, memo) {
                    return true;
                }
            }

            // left
            if x > 0 {
                if Self::helper(word, board, x - 1, y, index, memo) {
                    return true;
                }
            }

            // up
            if y > 0 {
                if Self::helper(word, board, x, y - 1, index, memo) {
                    return true;
                }
            }

            // down
            if y + 1 < board.len() {
                if Self::helper(word, board, x, y + 1, index, memo) {
                    return true;
                }
            }

            // all routes failed, so backtrack
            *index -= 1;
            memo[y][x] = false;
        }

        false
    }

    fn get_board(height: usize, width: usize) -> Vec<Vec<bool>> {
        let mut memo = Vec::with_capacity(height);
        for _ in 0..height {
            memo.push(std::iter::repeat(false).take(width).collect());
        }
        memo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let mut board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let expected = true;
        let ans = Solution::exist(board, word);
        assert_eq!(expected, ans);
    }
}

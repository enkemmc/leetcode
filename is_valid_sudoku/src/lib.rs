impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[false;9];9];
        let mut cols = [[false;9];9];
        let mut boxes = [[false;9];9];

        for y in 0..9 {
            for x in 0..9 {
                let i = board[y][x];
                
                if i != '.' {
                    let i = (i as u8 - 49) as usize;
                    if !rows[y][i] && !cols[x][i] && !boxes[(y/3) * 3 + (x/3)][i] { 
                        rows[y][i] = true;
                        cols[x][i] = true;
                        boxes[3* (y/3)+(x/3)][i] = true;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
let input = vec![["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]];
        let input:Vec<Vec<char>> = input
            .into_iter()
            .map(|a| a.into_iter().map(|st| {
                st.chars().nth(0).unwrap() as char
                }).collect::<Vec<char>>()
                 )
            .collect();
                 

            

        let ans = Solution::is_valid_sudoku(input);
        let expected = true;
        assert_eq!(expected, ans);
    }
}

struct Solution;

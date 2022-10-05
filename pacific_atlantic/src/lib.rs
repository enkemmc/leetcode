/ (r, c) == (y, x)
// rules:
// 1) either one or both of cells south and east, or north and west of the cell are smaller than it

// solution is 4MN speed
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answers = vec![];
        for r in 0..heights.len() {
            for c in 0..heights[r as usize].len() {
                if Self::is_peak(&heights, r, c) {
                    answers.push(vec![r as i32, c as i32]);
                }
            }
        }

        answers
    }

    fn is_peak(heights: &Vec<Vec<i32>>, r: usize, c: usize) -> bool {
        let val = heights[r][c];
        let l = if c == 0 {
            0
        } else {
            heights[r][c-1]
        };
        let u = if r == 0 {
            0
        } else {
            heights[r-1][c]
        };
        let _r = if c == heights[0].len() - 1 {
            0
        } else {
            heights[r][c+1]
        };
        let d = if r == heights.len() - 1 {
            0
        } else {
            heights[r+1][c]
        };

        (val > l || val > u) && (val > _r || val > d)
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

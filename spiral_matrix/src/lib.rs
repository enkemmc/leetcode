impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        use Direction::*;
        let mut x_axis = 0..matrix[0].len();
        let mut y_axis = 0..matrix.len();
        let mut directions = [Right,Down,Left,Up].into_iter().cycle();

        let mut nums = Vec::with_capacity(x_axis.end * y_axis.end);
        while !x_axis.is_empty() && !y_axis.is_empty() {
            match directions.next().unwrap() {
                Right => {
                    for x in x_axis.clone() {
                        nums.push(matrix[y_axis.start][x]);
                    }
                    y_axis.start += 1;
                },
                Left => {
                    for x in x_axis.clone().rev() {
                        nums.push(matrix[y_axis.end-1][x]);
                    }
                    y_axis.end -= 1;
                },
                Down => {
                    for y in y_axis.clone() {
                        nums.push(matrix[y][x_axis.end-1]);
                    }
                    x_axis.end -= 1;
                },
                Up => {
                    for y in y_axis.clone().rev() {
                        nums.push(matrix[y][x_axis.start]);
                    }
                    x_axis.start += 1;
                },
            }
        }
        nums
    }
}

#[derive(Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let matrix = vec!
        [[1,2,3],[4,5,6],[7,8,9]]
        .into_iter().map(|a| a.to_vec()).collect();
        let ans = Solution::spiral_order(matrix);
        let expected = vec![1,2,3,6,9,8,7,4,5];
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let matrix = vec!
        [[3],[2]]
        .into_iter().map(|a| a.to_vec()).collect();
        let ans = Solution::spiral_order(matrix);
        let expected = vec![3,2];
        assert_eq!(ans, expected);
    }
}

struct Solution;

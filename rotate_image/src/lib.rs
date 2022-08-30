type Matrix = Vec<Vec<i32>>;
type Point = (usize, usize);

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut level = 0;
        let len = matrix.len();
        // (y, x)

        while level < len / 2 {
            let mut t = (0,0);
            let mut r = (0, len-1);
            let mut b = (len-1,len-1);
            let mut l = (len-1,0);
            t.0 += level;
            t.1 += level;
            r.0 += level;
            r.1 -= level;
            b.0 -= level;
            b.1 -= level;
            l.0 -= level;
            l.1 += level;
            let mut points = vec![t, r, b, l];
            for _ in 0..len-1-(level*2) {
                Self::swap(matrix, &mut points);
                Self::increment_points(&mut points);
            }
            level += 1;
        }
    }

    pub fn swap(matrix: &mut Matrix, points: &mut Vec<Point>) {
        let mut temp = 0;

        for p in points.iter_mut().rev() {
            std::mem::swap(&mut matrix[p.1][p.0], &mut temp);
        }

        let first = points[3];
        std::mem::swap(&mut matrix[first.1][first.0], &mut temp);
    }

    pub fn increment_points(points: &mut Vec<Point>) {
        points[0].1 += 1;
        points[1].0 += 1;
        points[2].1 -= 1;
        points[3].0 -= 1;
    }

    pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
        for row in matrix {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut matrix = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ];
        Solution::print_matrix(&matrix);
        Solution::rotate(&mut matrix);
        println!();
        Solution::print_matrix(&matrix);
        assert_eq!(true, true);
    }
}

struct Solution;

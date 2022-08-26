#[derive(Debug)]
struct Point(usize, usize); // (y, x)

impl Solution {
    // 
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let y = image.len();
        let x = image[0].len();
        let base_color = image[sr as usize][sc as usize];
        let mut stack = vec![Point(sr, sc)];
        while let Some(point) = stack.pop() {
            Self::add_valid(&mut image, &mut stack, color, base_color, point);
        }
        image
    }

    // adds valid points to queue and fills current point
    fn add_valid(image: &mut Vec<Vec<i32>>, stack: &mut Vec<Point>, color: i32, base_color: i32, point: Point) {
        let Point(y, x) = point;

        if image[y][x] == base_color {
            image[y][x] = color;
            // L U R D
            if x > 0 {
                stack.push(Point(y, x - 1));
            }
            if y > 0 {
                stack.push(Point(y - 1,x));
            }
            if x + 1 < image[0].len() {
                stack.push(Point(y, x + 1));
            }
            if y + 1 < image.len() {
                stack.push(Point(y +1, x));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn first() {
        let input = vec![[1,1,1],[1,1,0],[1,0,1]].into_iter().map(|v| v.to_vec()).collect();
        let sr = 1;
        let sc = 1;
        let color = 2;
        let ans = Solution::flood_fill(input, sr, sc, color);
        let expect: Vec<Vec<i32>> = vec![[2,2,2],[2,2,0],[2,0,1]].into_iter().map(|v| v.to_vec()).collect();
        assert_eq!(expect, ans);
    }

    #[test]
    fn second(){
        let input = vec![[0,0,0],[0,0,0]].into_iter().map(|v| v.to_vec()).collect();
        let sr = 1;
        let sc = 1;
        let color = 2;
        let ans = Solution::flood_fill(input, sr, sc, color);
        let expect: Vec<Vec<i32>> = vec![[2,2,2],[2,2,2]].into_iter().map(|v| v.to_vec()).collect();
        assert_eq!(expect, ans);
    }
}

struct Solution;

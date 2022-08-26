// north = 0
// east = 1
// south = 2
// west = 3
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {

        let mut x = 0;
        let mut y = 0;
        let mut heading = 0;

        for c in instructions.chars() {
            match c {
                'L' => {
                    heading = (heading + 1) % 4;
                },
                'R' =>  {
                    if heading == 0 {
                        heading = 3;
                    } else {
                        heading -= 1;
                    }
                },
                'G' => {
                    Self::go(&mut x, &mut y, heading)
                },
                _ => unreachable!()
            }
        }
        (x == 0 && y == 0) || heading != 0
    }

    #[inline]
    fn go(x: &mut i32, y: &mut i32, heading: u8) {
        match heading {
            0 => *y += 1,
            1 => *x += 1, 
            2 => *y -= 1,
            3 => *x -= 1,
            _ => unreachable!()
        }
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

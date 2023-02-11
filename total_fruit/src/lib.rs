impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        //[1,0,1,1,2,2,1]
        let mut l = 0;
        let (mut first_start, mut first_end, mut second_start, mut second_end) = (None, None, None, None);
        let mut ans = 0;

        for (r, &fruit) in fruits.iter().enumerate() {
            ans = ans.max(r - l);
            //fill the baskets
            //if fruit cant fit
            //  dump one basket
            match (first_start, second_start){
                (Some(first_l), Some(second_l)) => {
                    if fruit != fruits[first_l] && fruit != fruits[second_l] {
                        // which basket do we drop?  the one that isn't r-i
                        if fruits[first_l] == fruits[r-1] {
                            // drop second
                            l = second_end.unwrap() + 1;
                            second_start = Some(r);
                            second_end = Some(r);
                        } else {
                            // drop first
                            l = first_end.unwrap() + 1;
                            first_start = Some(r);
                            first_end = Some(r);
                        }
                    } else {
                        if fruits[first_l] == fruit {
                            first_end = Some(r);
                        } else {
                            second_end = Some(r);
                        }
                    }
                },
                (Some(first_l), None) => {
                    if fruits[first_l] != fruit {
                        second_start = Some(r);
                        second_end = Some(r);
                    } else {
                        first_end = Some(r);
                    }
                },
                (None, Some(second_l)) => {
                    if fruits[second_l] != fruit {
                        first_start = Some(r);
                        first_end = Some(r);
                    } else {
                        second_end = Some(r);
                    }
                },
                (None, None) => {
                    first_start = Some(r);
                    first_end = Some(r);
                }
            }
        }
        ans = ans.max(fruits.len() - l);

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let fruits = vec![1,2,1];
        println!("{:?}", fruits);
        let expected = 3;
        let ans = Solution::total_fruit(fruits);
        assert_eq!(ans, expected);
    }


    #[test]
    fn second() {
        let fruits = vec![0,1,2,2];
        println!("{:?}", fruits);
        let expected = 3;
        let ans = Solution::total_fruit(fruits);
        assert_eq!(ans, expected);
    }

    #[test]
    fn third() {
        let fruits = vec![1,2,3,2,2];
        println!("{:?}", fruits);
        let expected = 4;
        let ans = Solution::total_fruit(fruits);
        assert_eq!(ans, expected);
    }

    #[test]
    fn fourth() {
        let fruits = vec![0,0,1,1];
        println!("{:?}", fruits);
        let expected = 4;
        let ans = Solution::total_fruit(fruits);
        assert_eq!(ans, expected);
    }

    #[test]
    fn fifth() {
        let fruits = vec![1,0,1,1,2,2,1];
        println!("{:?}", fruits);
        let expected = 5;
        let ans = Solution::total_fruit(fruits);
        assert_eq!(ans, expected);
    }

    #[test]
    fn sixth() {
        let fruits = vec![3,3,3,1,2,1,1,2,3,3,4];
        println!("{:?}", fruits);
        let expected = 5;
        let ans = Solution::total_fruit(fruits);
        assert_eq!(ans, expected);
    }

}

struct Solution;

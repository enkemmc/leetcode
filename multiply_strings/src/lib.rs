struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut chars1 = num1.chars().rev().collect::<Vec<char>>();
        let mut chars2 = num2.chars().rev().collect::<Vec<char>>();
        let mut product = 0;

        for (i1, ch1) in chars1.iter().enumerate(){
            for (i2, ch2) in chars2.iter().enumerate(){
                let mut o = Self::mult_ch_byte(*ch1, *ch2);
                if i1 + i2 > 0 {
                    let scalar = i1 + i2;
                    o *= 10 * scalar;
                }
                // scale o
                product += o;
            }
        }

        product.to_string()
    }

    fn mult_ch_byte(n1: char, n2: char) -> usize {
        println!("multiplying {} x {}", n1, n2);
        (n1 as usize - 48) * (n2 as usize - 48)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ans = Solution::multiply("2".to_string(), "30".to_string());
        let expected = "60";
        assert_eq!(ans, expected);
    }

    // #[test]
    // fn test_mult_fn(){
    //     let five = '5';
    //     let six = '6';
    //     let ans = Solution::mult_ch_byte(five, six);
    //     assert_eq!(ans, 30);
    // }
}
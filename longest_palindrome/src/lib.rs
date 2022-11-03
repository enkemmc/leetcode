use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut pair_to_count:HashMap<String, usize> = HashMap::new();
        let mut count = 0;
        let mut doubles = HashSet::new();

        for word in words {
            let inverse = String::from_utf8(word.clone().bytes().rev().collect::<Vec<u8>>()).unwrap();
            if Self::is_double(&word) {
                doubles.insert(word.clone());
            }
            if let Some(value) = pair_to_count.get_mut(&inverse){ 
                if *value > 0 {
                    *value -= 1;
                    count += 4;
                } else {
                    *pair_to_count.entry(word).or_insert(0) += 1;
                }
            } else {
                *pair_to_count.entry(word).or_insert(0) += 1;
            }
        }

        for s in doubles {
            if let Some(&v) = pair_to_count.get(&s) {
                if v > 0 {
                    count += 2;
                    break;
                }
            }
        }

        count
    }

    fn is_double(s: &str) -> bool {
        (s.get(0..1), s.get(1..)) == (s.get(1..), s.get(0..1))
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let s = "aa";
        let one = s.get(0..1);
        let two = s.get(1..);
        let c = one == two;
        println!("{:?}", one);
        println!("{:?}", two);
        println!("{:?}", c);
        
        assert_eq!(4, 4);
    }
}

struct Solution;

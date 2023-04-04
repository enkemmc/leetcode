use std::cmp::Ordering;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        let mut ans = Vec::with_capacity(spells.len());
        for &spell in &spells {
            let i = binary_search(spell, &potions, success);
            let value = (potions.len() - i) as i32;
            ans.push(value);
        }
        ans
    }
}

// find the index of the first potion that results in a success
fn binary_search(spell: i32, potions: &[i32], target: i64) -> usize {
    let mut l = 0;
    let mut r = potions.len();
    let mut mid;
    let mut product;
    while l < r {
        mid = (l + r) / 2;
        product = spell as i64 * potions[mid] as i64;
        match product.cmp(&target) {
            Ordering::Equal | Ordering::Greater => {
                r = mid;
            },
            Ordering::Less => {
                l = mid+1;
            }
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let spells = vec![5,1,3];
        let potions = vec![1,2,3,4,5];
        let target = 9;
        let i = binary_search(spells[2], &potions, target);
        let expected = 2;
        assert_eq!(i, expected);
    }

    fn it_works() {
        let spells = vec![5,1,3];
        let potions = vec![1,2,3,4,5];
        let success = 7;
        let ans = Solution::successful_pairs(spells, potions, success);
        let expected = vec![4,0,3];
        assert_eq!(ans, expected);
    }

    fn second() {
        let spells = vec![15,8,19];
        let potions = vec![38,36,23];
        let success = 328;
        let ans = Solution::successful_pairs(spells, potions, success);
        let expected = vec![4,0,3];
        assert_eq!(ans, expected);
    }

    fn third() {
        let spells = vec![13,22,21,13,11,9,13,35,7,38,10,10,38,19,3,16,13,24,16,27,20,24,32,5,16,35,24,2,25,32,20,22,22,3,35,39,27,26,25,21,27,40,15,17,24,40,35,27,20,40,9,35,27,19,15,34,35,37,17,40,8,3,33,39,29,22,30,1,37,2,16,30,32,31,24,6,34,26,36,4,21,2,29,31,3,27,6,24,40,18];
        let potions = vec![33,16,35,14,26,23,23,2,37,23,15,20,25,34,23,29,4,18,26,24,16,37,15,11,33,24,12,13,7,24,3,26,1,3,38,33,19,3,34,22,30,39,18,7,21,4,33,18,39,5,34,14,32,5,20,22,5,25,15];
        let success = 533;
        let ans = Solution::successful_pairs(spells, potions, success);
        let expected = vec![0,21,19,0,0,0,0,39,0,42,0,0,42,16,0,9,0,28,9,33,16,28,37,0,9,39,28,0,30,37,16,21,21,0,39,44,33,31,30,19,33,44,5,14,28,44,39,33,16,44,0,39,33,16,5,39,39,42,14,44,0,0,37,44,34,21,37,0,42,0,9,37,37,37,28,0,39,31,42,0,19,0,34,37,0,33,0,28,44,15];
        assert_eq!(ans, expected);
    }
}

struct Solution;

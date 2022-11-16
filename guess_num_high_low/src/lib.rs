/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(max: i32) -> i32 {
        let mut bounds = 1..max;
        let mut mid = max / 2;

        loop {
            match guess(mid) {
                -1 => {
                    bounds.end = mid - 1;
                    mid = bounds.start + (bounds.end - bounds.start) / 2
                },
                0 => return mid,
                1 => {
                    bounds.start = mid + 1;
                    mid = bounds.start + (bounds.end - bounds.start) / 2
                },
                _ => unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn first() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Solution;

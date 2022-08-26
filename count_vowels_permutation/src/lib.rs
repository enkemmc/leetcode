// [a, e, i, o, u]
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut ending_count = [1u64, 1u64, 1u64, 1u64, 1u64];
        let MOD = 1000000007;

        for _ in 1..n {
            let mut new_count = [0, 0, 0, 0, 0];
            // a
            new_count[1] += ending_count[0] % MOD;

            // e
            new_count[0] += ending_count[1] % MOD;
            new_count[2] += ending_count[1] % MOD;

            // i
            new_count[0] += ending_count[2] % MOD;
            new_count[1] += ending_count[2] % MOD;
            new_count[3] += ending_count[2] % MOD;
            new_count[4] += ending_count[2] % MOD;

            // o
            new_count[2] += ending_count[3] % MOD;
            new_count[4] += ending_count[3] % MOD;

            // u
            new_count[0] += ending_count[4] % MOD;

            ending_count = new_count;
        }

        let mut ans = 0;
        for count in ending_count {
            ans += count;
        }
        (ans % MOD) as i32
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

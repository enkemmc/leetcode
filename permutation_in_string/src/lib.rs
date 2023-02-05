
// is there a contiguous substring within s2 that == a permutation of s1
// skip generating permutations by matching against char : count

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let len = s1.len();
        let s2: Vec<u8> = s2.bytes().collect();

        let mut availability = Availability::new(&s1);
        let original = availability.clone();

        let mut l = 0;
        let mut counting = false;

                // "abcc"
                // "oobacababc"
        for (r, &b) in s2.iter().enumerate() {
            if !counting {
                // if it matches, check availability and
                if availability.is_available(b){
                    availability.sub(b);
                    counting = true;
                }
                l = r;
            } else {
                // if it does match, check avail and continue
                if availability.is_available(b){
                    availability.sub(b);
                } else {
                    // if its not avail, and it doesnt exist in s1, l=r and restart availa
                    if !original.is_available(b) {
                        availability = original.clone();
                        counting = false;
                    } else {
                        // if its not avail, move l up to the index+1 of the next b
                        'a:
                        while l <= r {
                            if s2[l] == b {
                                availability.add(b);
                                l += 1;   
                                break 'a;     
                            } else {
                                availability.add(s2[l]);
                                l += 1;
                            }
                        }
                        // l should now be pointing at the character after b
                        availability.sub(b);
                    }
                }
            }

            if counting && (r + 1 - l) == len {
                return true;
            }
        }

        counting && s2.len() - l == len
    }
}

#[derive(Clone)]
struct Availability {
    inner: [usize;26]
}

impl Availability {
    fn new(s: &str) -> Self {
        let mut inner = [0;26];
        for b in s.bytes() {
            inner[b as usize - 97] +=1;
        }
        Self {
            inner
        }
    }

    fn is_available(&self, b: u8) -> bool { 
        self.inner[b as usize - 97] > 0
    }

    fn add(&mut self, b: u8) {
        self.inner[b as usize - 97] += 1;
    }

    fn sub(&mut self, b: u8) {
        self.inner[b as usize - 97] -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn first() {
        // "abcc"
        // "oobacababc"
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        let ans = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(ans, expected);
    }

    fn second() {
        // "abcc"
        // "oobacababc"
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        let ans = Solution::check_inclusion(s1, s2);
        let expected = false;
        assert_eq!(ans, expected);
    }


    fn third() {
        // "abcc"
        // "oobacababc"
        let s1 = "hello".to_string();
        let s2 = "ooolleoooleh".to_string();
        println!("s1: {}", s1);
        println!("s2: {}", s2);
        let ans = Solution::check_inclusion(s1, s2);
        let expected = false;
        assert_eq!(ans, expected);
    }

    #[test]
    fn fourth() {
        // "abcc"
        // "oobacababc"
        let s1 = "a".to_string();
        let s2 = "b".to_string();
        println!("s1: {}", s1);
        println!("s2: {}", s2);
        let ans = Solution::check_inclusion(s1, s2);
        let expected = false;
        assert_eq!(ans, expected);
    }

}

struct Solution;

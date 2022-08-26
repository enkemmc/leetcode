use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut used = vec![false;num_courses as usize];
        let mut num_to_prereqs = HashMap::new(); // use to look up set of prereqs for each course
        let mut courses = Vec::with_capacity(num_courses as usize);
        for p in prerequisites {
            num_to_prereqs.entry(p[0] as usize).or_insert_with(HashSet::new).insert(p[1] as usize);
        }
        for num in 0..num_courses as usize { // course_num
            // add this number's prereqs before it
            if let Some(prereqs) = num_to_prereqs.get(&num) {
                for p in prereqs {
                    // check if the num is a prereq of one of its own prereqs
                    if let Some(hs) = num_to_prereqs.get(p) {
                        if hs.contains(&num) {
                            return vec![];
                        }
                    }
                    // check if its already been included in the course schedule'
                    if !used[*p] {
                        courses.push(*p as i32);
                        used[*p] = true;
                    }
                }
                if !used[num] {
                    courses.push(num as i32);
                    used[num] = true;
                }
            }
        }
        for (i, &used) in used.iter().enumerate() {
            if !used {
                courses.push(i as i32);
            }
        }
        courses
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn first() {
        let input = vec![[1,0],[2,0],[3,1],[3,2]].into_iter().map(|v| v.to_vec()).collect();
        let expected = vec![0,2,1,3];
        let ans = Solution::find_order(4, input);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let input = vec![];
        let expected = vec![0];
        let ans = Solution::find_order(1, input);
        assert_eq!(ans, expected);
    }

}

struct Solution;

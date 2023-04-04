use std::cmp::Ordering;

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        fn get_dist(point: &[i32]) -> f64 {
            let [x,y] = point[..];
            (x.pow(2) as f64 + y.pow(2) as f64).sqrt()
        }
        points
            .sort_by(|p1, p2| {
                let d1 = get_dist(&p1);
                let d2 = get_dist(&p2);
                if d1 > d2 {
                    Ordering::Less
                } else if d2 > d1 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

        points
            .truncate(k as usize);
        points
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

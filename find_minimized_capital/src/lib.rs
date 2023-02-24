use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        // the heap contains all accessible projects, sorted by max profit
        let mut heap = BinaryHeap::new();
        let mut capital_to_profits = capital
            .into_iter()
            .zip(profits)
            .collect::<Vec<(i32,i32)>>();
        capital_to_profits.sort_unstable(); 
        let mut cp_iter = capital_to_profits.into_iter().peekable();
        // sorted lowest to highest by capital
        let mut p_count = 0;
        let mut cap_count = w;
        while p_count < k {
            // 1. try to load any new projects that have become available due to our current cap_count
            //    into the heap
            // 2. pop the heap to find the best project.
            // 3. increase p_count and cap_count w/ new project stats
            'cap_load:
            while let Some(&(req_cap, _profit)) = cp_iter.peek() {
                if cap_count >= req_cap {
                    heap.push(cp_iter.next().unwrap().1);
                } else {
                    break 'cap_load;
                }
            }

            if let Some(best) = heap.pop() {
                cap_count += best;
                p_count += 1;
            } else {
                break;
            }
        }
        cap_count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn first() {
        let k = 2;
        let w = 0;
        let profits = vec![1,2,3];
        let capital = vec![0,1,1];
        let expected = 4;
        let ans = Solution::find_maximized_capital(k, w, profits, capital);
        assert_eq!(ans, expected);
    }
}

struct Solution;

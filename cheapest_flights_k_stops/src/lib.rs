use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

// if we pop from the heap based on least steps not cheapest flight
// it should prevent us from polluting the cache with cheap paths that take too many steps
// then we can store cheapest paths to current location without worrying about it containing super
// cheap paths that take too many steps
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let dst = dst as usize;
        // airport : (airport, cost)
        let mut connections = vec![vec![];n];
        let mut location_to_cheapest_price = vec![i32::MAX;n];

        // min_heap. pop priority is: min_steps, min_cost, min_location_id?? lol
        // (steps, total_cost, current_location)
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((-1, 0, src as usize)));

        for flight in flights.iter() {
            if let [from, to, price] = flight[..] {
                connections[from as usize].push((price, to as usize));
            }
        }

        while let Some(Reverse(path)) = heap.pop() {
            let (steps, cost_so_far, location) = path;
            if steps > k {
                break;
            }
            if location_to_cheapest_price[location] > cost_so_far {
                location_to_cheapest_price[location] = cost_so_far;

                // load all connecting flights into the path queue
                for ticket in &connections[location] {
                    let (ticket_cost, dest) = ticket;
                    heap.push(Reverse((steps + 1,cost_so_far + ticket_cost, *dest)));
                }
            }
        }

        if location_to_cheapest_price[dst] == i32::MAX {
            -1
        } else {
            location_to_cheapest_price[dst]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first() {
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let n = 4;
        let src = 0;
        let dst = 3;
        let k = 1;
        let ans = Solution::find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(ans, 700);
    }

    #[test]
    fn second() {
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![0, 2, 500],
        ];
        let n = 3;
        let src = 0;
        let dst = 2;
        let k = 1;
        let ans = Solution::find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(ans, 200);
    }

    #[test]
    fn third() {
        let flights = vec![
            [0, 12, 28],
            [5, 6, 39],
            [8, 6, 59],
            [13, 15, 7],
            [13, 12, 38],
            [10, 12, 35],
            [15, 3, 23],
            [7, 11, 26],
            [9, 4, 65],
            [10, 2, 38],
            [4, 7, 7],
            [14, 15, 31],
            [2, 12, 44],
            [8, 10, 34],
            [13, 6, 29],
            [5, 14, 89],
            [11, 16, 13],
            [7, 3, 46],
            [10, 15, 19],
            [12, 4, 58],
            [13, 16, 11],
            [16, 4, 76],
            [2, 0, 12],
            [15, 0, 22],
            [16, 12, 13],
            [7, 1, 29],
            [7, 14, 100],
            [16, 1, 14],
            [9, 6, 74],
            [11, 1, 73],
            [2, 11, 60],
            [10, 11, 85],
            [2, 5, 49],
            [3, 4, 17],
            [4, 9, 77],
            [16, 3, 47],
            [15, 6, 78],
            [14, 1, 90],
            [10, 5, 95],
            [1, 11, 30],
            [11, 0, 37],
            [10, 4, 86],
            [0, 8, 57],
            [6, 14, 68],
            [16, 8, 3],
            [13, 0, 65],
            [2, 13, 6],
            [5, 13, 5],
            [8, 11, 31],
            [6, 10, 20],
            [6, 2, 33],
            [9, 1, 3],
            [14, 9, 58],
            [12, 3, 19],
            [11, 2, 74],
            [12, 14, 48],
            [16, 11, 100],
            [3, 12, 38],
            [12, 13, 77],
            [10, 9, 99],
            [15, 13, 98],
            [15, 12, 71],
            [1, 4, 28],
            [7, 0, 83],
            [3, 5, 100],
            [8, 9, 14],
            [15, 11, 57],
            [3, 6, 65],
            [1, 3, 45],
            [14, 7, 74],
            [2, 10, 39],
            [4, 8, 73],
            [13, 5, 77],
            [10, 0, 43],
            [12, 9, 92],
            [8, 2, 26],
            [1, 7, 7],
            [9, 12, 10],
            [13, 11, 64],
            [8, 13, 80],
            [6, 12, 74],
            [9, 7, 35],
            [0, 15, 48],
            [3, 7, 87],
            [16, 9, 42],
            [5, 16, 64],
            [4, 5, 65],
            [15, 14, 70],
            [12, 0, 13],
            [16, 14, 52],
            [3, 10, 80],
            [14, 11, 85],
            [15, 2, 77],
            [4, 11, 19],
            [2, 7, 49],
            [10, 7, 78],
            [14, 6, 84],
            [13, 7, 50],
            [11, 6, 75],
            [5, 10, 46],
            [13, 8, 43],
            [9, 10, 49],
            [7, 12, 64],
            [0, 10, 76],
            [5, 9, 77],
            [8, 3, 28],
            [11, 9, 28],
            [12, 16, 87],
            [12, 6, 24],
            [9, 15, 94],
            [5, 7, 77],
            [4, 10, 18],
            [7, 2, 11],
            [9, 5, 41],
        ]
        .into_iter()
        .map(|v| v.to_vec())
        .collect();
        let n = 17;
        let src = 13;
        let dst = 4;
        let k = 13;
        let ans = Solution::find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(ans, 47);
    }
}

struct Solution;

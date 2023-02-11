use std::collections::VecDeque;

enum Color {
    Red,
    Blue
}

impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        use Color::*;

        // adjacencies[node_num][0] == vec![index_of_red_nodes...]
        let mut adjacencies = vec![[vec![],vec![]];n as usize]; // [reds, blues]
        for edge in red_edges {
            adjacencies[edge[0] as usize][0].push(edge[1] as usize);
        }
        for edge in blue_edges {
            adjacencies[edge[0] as usize][1].push(edge[1] as usize);
        }
        
        // stores shortest path from 0..i  ->  (PrevNodeRed, PrevNodeBlue)
        let mut shortest_path: Vec<(Option<usize>, Option<usize>)> = vec![(None, None);n as usize];

        // stores states
        // state = (step_count, last_color, prev_location)
        let mut queue = VecDeque::new();
        queue.push_back((0, Red, 0));
        queue.push_back((0, Blue, 0));

        while let Some((count, last_color, location)) = queue.pop_front() {
            if location == n as usize {
                continue;
            }
            match last_color {
                Red => {
                    // check if this count is less than min red
                    if let Some(min) = shortest_path[location].0.as_mut() {
                        if count < *min {
                            *min = count;
                            for &new_loc in &adjacencies[location][1] {
                                queue.push_back((count + 1, Blue, new_loc));
                            }
                        } 
                    } else {
                        shortest_path[location].0 = Some(count);
                        for &new_loc in &adjacencies[location][1] {
                            queue.push_back((count + 1, Blue, new_loc));
                        }
                    }
                },
                Blue => {
                    if let Some(min) = shortest_path[location].1.as_mut() {
                        if count < *min {
                            // now check if there are any paths from this index to another node
                            // that are red
                            *min = count;
                            for &new_loc in &adjacencies[location][0] {
                                queue.push_back((count + 1, Red, new_loc));
                            }
                        } 
                    } else {
                        shortest_path[location].1 = Some(count);
                        for &new_loc in &adjacencies[location][0] {
                            queue.push_back((count + 1, Red, new_loc));
                        }
                    }
                }
            }
        }

        shortest_path
            .into_iter()
            .map(|(red_min, blue_min)| {
                match (red_min, blue_min) {
                    (None, None) => -1i32,
                    (Some(r), Some(b)) => r.min(b) as i32,
                    (Some(r), None) => r as i32,
                    (None, Some(b)) => b as i32
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    #[test]
    fn first() {
        let n = 3;
        let red_edges = vec![
            [0,1],[1,2]
        ]
            .into_iter()
            .map(|a| a.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let blue_edges = vec![];
        //let blue_edges: Vec<Vec<i32>> = vec![

        //]
        //    .into_iter()
        //    .map(|a| a.to_vec())
        //    .collect::<Vec<Vec<i32>>>();
        let expected = vec!
            [0,1,-1]
            ;
        let ans = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
        assert_eq!(ans, expected);
    }

    #[test]
    fn second() {
        let n = 3;
        let red_edges = vec![
            [0,1],[0,2]
        ]
            .into_iter()
            .map(|a| a.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let blue_edges: Vec<Vec<i32>> = vec![
            [1,0]
        ]
            .into_iter()
            .map(|a| a.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let expected = vec!
            [0,1,1]
            ;
        let ans = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
        assert_eq!(ans, expected);
    }
}

struct Solution;

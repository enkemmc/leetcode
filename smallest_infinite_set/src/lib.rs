use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

struct SmallestInfiniteSet {
    added_back: BinaryHeap<Reverse<i32>>,
    next: i32,
    removed: HashSet<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self {
            added_back: BinaryHeap::new(),
            next: 1,
            removed: HashSet::new()
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(n)) = self.added_back.pop() {
            self.removed.insert(n);
            n
        } else {
            self.removed.insert(self.next);
            self.next += 1;
            self.next -1
        }
    }
    
    fn add_back(&mut self, num: i32) {
        if self.removed.contains(&num) {
            self.removed.remove(&num);
            self.added_back.push(Reverse(num));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let mut smallest = SmallestInfiniteSet::new();
        assert_eq!(1, smallest.pop_smallest());
        assert_eq!(2, smallest.pop_smallest());
        smallest.add_back(1);
        assert_eq!(1, smallest.pop_smallest());
    }
}

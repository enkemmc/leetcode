use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        let mut ans = vec![];
        let mut largest = i32::MIN;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let curr = queue.pop_front().unwrap();
                largest = largest.max(curr.borrow().val);
                if curr.borrow().left.is_some() {
                    queue.push_back(curr.borrow_mut().left.take().unwrap());
                }
                if curr.borrow().right.is_some() {
                    queue.push_back(curr.borrow_mut().right.take().unwrap());
                }
            }
            ans.push(largest);
            largest = i32::MAX;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

struct Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

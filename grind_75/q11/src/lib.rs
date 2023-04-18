// Definition for a binary tree node.
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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;


impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut max_height = i32::MAX;
        let mut curr_height = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            if curr_height - max_height > 1 {
                return false;
            }
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let borrow = node.borrow();
                match (borrow.left.as_ref(), borrow.right.as_ref()) {
                    (Some(l), Some(r)) => {
                        queue.push_back(Rc::clone(l));
                        queue.push_back(Rc::clone(r));
                    },
                    (Some(l), None) => queue.push_back(Rc::clone(l)),
                    (None, Some(r)) => queue.push_back(Rc::clone(r)),
                    (None, None) =>  {
                        max_height = curr_height;
                    }
                }
            }      
            curr_height += 1;
        }
        true
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

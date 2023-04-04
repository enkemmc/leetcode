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
use Direction::*;
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        let mut dir = Right;
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let mut row = vec![];
            match dir {
                Left => {
                    for _ in 0..queue.len() {
                        if let Some(wrapped) = queue.pop_back().unwrap() {
                            let node = Rc::try_unwrap(wrapped).unwrap().into_inner();
                            row.push(node.val);
                            queue.push_front(node.right);
                            queue.push_front(node.left);
                        }
                    }
                    if !row.is_empty() {
                        ans.push(row);
                    }
                    dir = Right;
                },
                Right => {
                    for _ in 0..queue.len() {
                        if let Some(wrapped) = queue.pop_front().unwrap() {
                            let node = Rc::try_unwrap(wrapped).unwrap().into_inner();
                            row.push(node.val);
                            queue.push_back(node.left);
                            queue.push_back(node.right);
                        }
                    }
                    if !row.is_empty() {
                        ans.push(row);
                    }
                    dir = Left;
                }
            }
        }

        ans
    }
}

enum Direction {
    Left,
    Right
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

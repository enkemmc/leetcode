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
use std::collections::HashSet;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut compliments = HashSet::new();

        Self::helper(root, k, &mut  compliments)
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, k: i32, compliments: &mut HashSet<i32>) -> bool {
        if let Some(node) = root {
            let target = k - node.borrow().val;
            if compliments.contains(&target) {
                true
            } else {
                compliments.insert(node.borrow().val);
                if Self::helper(node.borrow().left.clone(), k, compliments) || Self::helper(node.borrow().right.clone(), k, compliments) {
                    true
                } else {
                    false
                }
            }
        } else {
            false
        }
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

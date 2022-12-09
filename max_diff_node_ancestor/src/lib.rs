use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{min, max};

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;
// this isnt a bst, so cant assume all nodes to right or left are necessarily > or < than all
// ancestors

// strategy:
// keep track of smallest and largest ancestor for each path
// if this diff is bigger, update max_diff
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let initial = root.as_ref().unwrap().borrow().val;
        let mut max_diff = 0;
        Self::helper(&root, &mut max_diff, initial, initial);
        max_diff 
    }

    fn helper(maybe_node: &MaybeNode, max_diff: &mut i32, s_ancest: i32, l_ancest: i32) {
        *max_diff = max(*max_diff, Self::abs_diff(s_ancest, l_ancest));
        if let Some(node) = maybe_node {
            let val = node.borrow().val;
            Self::helper(&node.borrow().left, max_diff, min(s_ancest, val), max(l_ancest, val));
            Self::helper(&node.borrow().right, max_diff, min(s_ancest, val), max(l_ancest, val));
        }
    }
    // abs_diff is still nightly so we'll build our own :)
    fn abs_diff(s_ancest: i32, l_ancest: i32) -> i32 {
        if s_ancest > l_ancest {
            s_ancest - l_ancest 
        } else {
            l_ancest - s_ancest 
        }
    }
}

// -

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Solution;

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

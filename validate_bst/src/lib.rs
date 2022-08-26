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

//                   5
//              4        6
//          n      n  3     7
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{min, max};
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root, i32::MAX, i32::MIN)
    }

    fn dfs(o: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> bool {
        if let Some(rc) = o {
            let l = rc.borrow().left.clone();
            let r = rc.borrow().right.clone();
            // check that val is less than min and greater than max
            let val = rc.borrow().val;
            if val > low {
                return false;
            }
            if val < high {
                return false;
            }

            match (l.clone(), r.clone()) {
                (Some(left), Some(right)) => {
                    if left.borrow().val < val && right.borrow().val > val {
                        Self::dfs(l, min(low, val), max(high, val)) && Self::dfs(r, min(low, val), max(high, val))
                    } else {
                        false
                    }
                },
                (None, Some(right)) => {
                    if right.borrow().val > val {
                        Self::dfs(r, min(low, val), max(high, val))
                    } else {
                        false
                    }
                },
                (Some(left), None) => {
                    if left.borrow().val < val {
                        Self::dfs(l, min(low, val), max(high, val))
                    } else {
                        false
                    }
                },
                (None, None) => true,
            }
        } else {
            true
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

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
type MaybeNode = Option<Rc<RefCell<TreeNode>>>;
enum Finding {
    Done(MaybeNode),
    FoundOne,
    FoundNone,
}

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

        if let Finding::Done(ans) = Self::helper(&root, p.unwrap().borrow().val, q.unwrap().borrow().val) {
            return ans;
        }
        unreachable!()
    }

    fn helper(root: &MaybeNode, p: i32, q: i32) -> Finding {
        use Finding::*;

        if let Some(node) = root {
            let this = if node.borrow().val == p || node.borrow().val == q { 
                FoundOne
            } else {
                FoundNone
            };

            let l = Self::helper(&node.borrow().left, p, q);
            if let Done(_) = l {
                return l;
            }
            let r = Self::helper(&node.borrow().right, p, q);

            match (l,r) {
                (FoundOne, FoundOne) => Done(root.clone()),
                (FoundOne, FoundNone) | (FoundNone, FoundOne) => {
                    match this {
                        FoundOne => Done(root.clone()),
                        _ => FoundOne,
                    }
                },
                (Done(n), _) | (_, Done(n)) => Done(n),
                (FoundNone, FoundNone) => {
                    this
                },
            }
        } else {
            FoundNone
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

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

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let mut stack = vec![];
        stack.push(root.clone());

        while let Some(maybe_node) = stack.pop() {
            if let Some(rcref) = maybe_node {
                let mut b = rcref.borrow_mut();
                if b.left.is_some() {
                    let t1 = b.left.take().unwrap();
                    let temp = b.right.replace(t1);
                    if let Some(wrapped) = temp {
                        b.left.replace(wrapped);
                    }
                } else if b.right.is_some() {
                    let t1 = b.right.take().unwrap();
                    let temp = b.left.replace(t1);
                    if let Some(wrapped) = temp {
                        b.right.replace(wrapped);
                    }
                }
                stack.push(b.left.clone());
                stack.push(b.right.clone());
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct Solution;

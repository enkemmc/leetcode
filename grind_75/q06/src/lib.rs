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
        let mut stack = vec![root.clone()];
        while let Some(opt) = stack.pop() {
            if let Some(mut node) = opt {
                Self::swap(&mut node);
                stack.push(node.borrow().left.clone());
                stack.push(node.borrow().right.clone());
            }
        }
        root
    }

    fn swap(node: &mut Rc<RefCell<TreeNode>>) {
        let l = node.borrow_mut().left.take();
        let r = node.borrow_mut().right.take();
        node.borrow_mut().left = r;
        node.borrow_mut().right = l;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn first() {
        let mut root = TreeNode { val: 0, left: None, right: None };
        root.left = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None })));
        root.right = Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None })));
        let mut root = Rc::new(RefCell::new(root));
        Solution::swap(&mut root);
        println!("{:#?}", root);
        assert_eq!(4, 4);
    }

}

struct Solution;

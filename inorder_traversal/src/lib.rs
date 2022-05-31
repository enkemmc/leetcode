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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = vec![];
        Self::helper(&mut values, &root);
        values
    }

    fn helper(values: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            {
                Self::helper(values, &node.borrow().left);
            }
            values.push(node.borrow().val);
            {
                Self::helper(values, &node.borrow().right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        //
        assert_eq!(4, 4);
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        Self::helper(&root, target_sum, &mut paths, &mut vec![], 0);
        paths
    }

    fn helper(
        node_wrapper: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        paths: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        sum: i32,
    ) {
        if let Some(node) = node_wrapper {
            let val = node.borrow().val;
            path.push(val);
            if sum + val == target && node.borrow().left.is_none() && node.borrow().right.is_none()
            {
                // exit condition
                paths.push(path.clone());
            } else {
                // check for node validity here, rather than at start
                if node.borrow().left.is_some() {
                    Self::helper(&node.borrow().left, target, paths, path, sum + val);
                }
                if node.borrow().right.is_some() {
                    Self::helper(&node.borrow().right, target, paths, path, sum + val);
                }
            }
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Solution;
use utils::TreeNode;

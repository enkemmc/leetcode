fn main() {
    let root = TreeNode::new(5);
    let ans = Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(root))));
}

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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums = vec![];
        let mut stack = vec![root.unwrap()];

        while let Some(rc) = stack.pop() {
            let TreeNode { val, left, right } = Rc::try_unwrap(rc).unwrap().into_inner();
            nums.push(val);
            if let Some(l) = left {
                stack.push(l);
            }
            if let Some(r) = right {
                stack.push(r);
            }
        }

        nums.sort_unstable();
        let mut min = i32::MAX;
        let mut prev = nums[0];
        for num in &nums[1..] {
            min = min.min(num - prev);
            prev = *num;
        }

        min
    }
}

struct Solution;

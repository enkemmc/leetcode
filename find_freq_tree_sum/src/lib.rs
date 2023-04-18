use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap};

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut num_to_count: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        helper(root, &mut num_to_count, &mut max);
        let mut ans = vec![];
        for (k,v) in num_to_count.into_iter() {
            if (v == max) {
                ans.push(k);
            }
        }
        ans
    }
}

fn helper(n: Option<Rc<RefCell<TreeNode>>>, num_to_count: &mut HashMap<i32, i32>, max: &mut i32) -> i32 {
    if let Some(node) = n {
        let b = node.borrow();
        let sum = b.val + helper(b.left.clone(), num_to_count, max) + helper(b.right.clone(), num_to_count, max);
        if let Some(count) = num_to_count.get_mut(&sum) {
            *count += 1;
            if *max < *count {
                *max = *count;
            }
        } else {
            num_to_count.insert(sum, 1);
            if *max == 0 {
                *max = 1;
            }
        }
        sum
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

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
      right: None
    }
  }
}

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

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut buffer = vec![];
        fn traverse(maybe_node: MaybeNode, stack: &mut Vec<i32>) -> i32 {
            if let Some(node) = maybe_node {
                let n = node.borrow();
                match (n.left.is_some(), n.right.is_some()) {
                    (false, false) => {
                        stack.push(n.val);
                        return Solution::sum_vec(stack);
                    },
                    (true, true) => {
                        stack.push(n.val);
                        let num = traverse(n.left.clone(), stack) + traverse(n.right.clone(), stack);
                        stack.pop();
                        return num;
                    },
                    (false, true) => {
                        stack.push(n.val);
                        let num = traverse(n.right.clone(), stack);
                        stack.pop();
                        return num;                    }
                    (true, false) => {
                        stack.push(n.val);
                        let num = traverse(n.left.clone(), stack);
                        stack.pop();
                        return num;
                    }
                }
            }
            unreachable!()
        }
        traverse(root, &mut buffer)
    }
    fn sum_vec(stack: &mut Vec<i32>) -> i32 {
        let mut sum = 0;
        let len = stack.len(); // [1,2,3] len = 3 end == len - 1 - i
        for i in 0..len {
            sum += stack[i] * 10i32.pow((len - 1 - i) as u32);
        }
        stack.pop();
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_vec_test() {
        let mut stack = vec![1,2,3];
        let ans = Solution::sum_vec(&mut stack);
        assert_eq!(ans, 123);
    }
}

struct Solution;

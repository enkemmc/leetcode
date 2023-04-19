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
type Maybe = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use State::*;
        let mut longest = i32::MIN;
        for dir in [Left, Right] {
            dfs(root.clone(), &mut longest, dir, 0);
        }
        longest
    }
}

fn dfs(root: Maybe, longest: &mut i32, dir: State, length: i32) {
    use State::*;
    if let Some(node) = root {
        let b = node.borrow();
        match dir {
            Left => {
                if b.left.is_some() {
                    dfs(b.left.clone(), longest, State::Right, length+1);
                } else {
                    if length > *longest {
                        *longest = length;
                    }
                }
                dfs(b.right.clone(), longest, State::Right, 0);
            },
            Right => {
                if b.right.is_some() {
                    dfs(b.right.clone(), longest, State::Left, length+1);
                } else {
                    if length > *longest {
                        *longest = length;
                    }
                }
                dfs(b.left.clone(), longest, State::Left, 0);
            }
        }
    }
}

enum State {
    Left,
    Right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

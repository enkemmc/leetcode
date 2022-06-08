use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut p_stack = vec![p];
        let mut q_stack = vec![q];
        match (p_stack.pop(), q_stack.pop()) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(maybe_p), Some(maybe_q)) => {
                // ensure they're both equal then push children onto stacks
                match (maybe_p, maybe_q) {
                    (Some(p), Some(q)) => {
                        if p.borrow().val == q.borrow().val {
                            // we're good.  add the children
                            p_stack.push(p.borrow().left.clone());
                            p_stack.push(p.borrow().right.clone());
                            q_stack.push(q.borrow().left.clone());
                            q_stack.push(q.borrow().right.clone());
                        } else {
                            return false;
                        }
                    }
                    (None, None) => (),
                    _ => return false,
                }
                true
            }
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

use utils::TreeNode;
struct Solution;

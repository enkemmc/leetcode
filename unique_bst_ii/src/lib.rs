use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

struct Solution;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode<i32>>>>> {
        let perms = Self::get_perms(&mut (1..=n).into_iter().collect::<Vec<i32>>());
        let mut trees = vec![];
        println!("{:?}", perms);
        if false {
            for perm in perms {
                Self::build_tree(perm, &mut trees);
            }
        }
        trees
    }

    fn build_tree(nums: Vec<i32>, trees: &mut Vec<Option<Rc<RefCell<TreeNode<i32>>>>>) {
        if nums.len() > 0 {
            let mut root = Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
            for num in &nums[1..] {
                Self::insert_node(&mut root, *num);
            }
            trees.push(root);
        }
    }

    fn insert_node(maybe_node: &mut Option<Rc<RefCell<TreeNode<i32>>>>, val: i32) {
        if let Some(node_ptr) = maybe_node.as_mut() {
            if val < node_ptr.borrow_mut().val {
                if node_ptr.borrow_mut().left.is_none() {
                    node_ptr.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::insert_node(&mut node_ptr.borrow_mut().left, val);
                }
            } else {
                if node_ptr.borrow_mut().right.is_none() {
                    node_ptr.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::insert_node(&mut node_ptr.borrow_mut().right, val);
                }
            }
        }
    }

    fn get_perms(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
        let mut perms = vec![];
        Self::helper(&mut perms, nums, 0);
        perms
    }

    fn helper(perms: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, k: usize) {
        if k == nums.len() {
            perms.push(nums.clone());
        } else {
            for i in k..nums.len() {
                nums.swap(i, k);
                Self::helper(perms, nums, k + 1);
                nums.swap(i, k);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let trees = Solution::generate_trees(3);
        println!("{:?}", trees);
        assert_eq!(4, 4);
    }
}

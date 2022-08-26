use utils::ListNode;
//Definition for singly-linked list.
//#[derive(PartialEq, Eq, Clone, Debug)]
//pub struct ListNode {
//  pub val: i32,
//  pub next: Option<Box<ListNode>>
//}
//
//impl ListNode {
//  #[inline]
//  fn new(val: i32) -> Self {
//    ListNode {
//      next: None,
//      val
//    }
//  }
//}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {

    }

}

struct Solution;

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn first(){
        let head = ListNode::from(vec![1,2,3,4,5]);
        let new_head = Solution::reverse_between(Some(Box::new(head)), 2, 4);
        let mut curr = new_head;
        while let Some(node) = curr {
            println!("{}", node.val);
            curr = node.next;
        }
        assert_eq!(true, true);
    }
}

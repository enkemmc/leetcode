use std::borrow::BorrowMut;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl From<Vec<i32>> for ListNode {
  fn from(v: Vec<i32>) -> Self {
     let mut temp = ListNode::new(0);
     let mut tar = &mut temp.next;
     for num in v {
      *tar = Some(Box::new(ListNode::new(num)));
      tar = &mut tar.as_mut().unwrap().next;
     }

     *temp.next.unwrap()
  }
}
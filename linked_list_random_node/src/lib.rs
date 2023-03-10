use rand::prelude::*;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {
    len: i32,
    head: Option<Box<ListNode>>,
    rng: ThreadRng
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut curr = &head;
        let mut len = 0i32;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        let rng = thread_rng();

        Self {
            len,
            head,
            rng
        }
    }
    
    fn get_random(&mut self) -> i32 {
        let mut curr = &self.head;
        let mut num = self.rng.gen_range(0..self.len);
        while let Some(node) = curr {
            if num == 0 {
                return node.val;
            } else {
                num -= 1;
                curr = &node.next;
            }
        }
        unreachable!()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

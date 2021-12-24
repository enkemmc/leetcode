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
struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut current= &mut *head.as_mut().unwrap();

        let mut l1 = l1;
        let mut l2 = l2;

        let mut carried = 0;

        while l1.is_some() || l2.is_some() {
            let n1 = if l1.is_some() {
                let n= l1.as_ref().unwrap().val;
                l1 = l1.unwrap().next;
                n
            } else {
                0
            };

            let n2 = if l2.is_some() {
                let n = l2.as_ref().unwrap().val;
                l2 = l2.unwrap().next;
                n
            } else {
                0
            };

            let mut sum = n1 + n2 + carried;
            carried = 0;

            if sum >= 10 {
                carried = 1;
                sum %= 10;
            }

            let new_node = ListNode::new(sum);
            current.next = Some(Box::new(new_node));
            current = &mut *current.next.as_mut().unwrap();
        }
        
        if carried == 1 {
            current.next = Some(Box::new(ListNode::new(1)));
        }

        head.unwrap().next
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let v1 = vec![2, 4, 3];
        let v2 = vec![5, 6, 4];

        let mut head = Solution::add_two_numbers(create_ll(&v1), create_ll(&v2));
        let mut output = vec![];
        while let Some(node) = head {
            output.push(node.val);
            head = node.next;
        }

        assert_eq!(output, vec![7, 0, 8]);
    }

    fn create_ll(nums: &[i32]) -> Option<Box<ListNode>> {
        if nums.is_empty() {
            None
        } else {
            let f = Box::new(ListNode::new(nums[0]));
            let mut head = Some(f);
            let mut current = &mut *head.as_mut().unwrap();

            for num in &nums[1..] {
                current.next = Some(Box::new(ListNode::new(*num)));
                current = current.as_mut().next.as_mut().unwrap();
            }

            head
        }
    }
}

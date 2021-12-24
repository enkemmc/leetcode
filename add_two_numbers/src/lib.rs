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
            let mut sum = match (&l1, &l2) {
                (Some(n1), Some(n2)) => n1.val + n2.val + carried,
                (None, Some(n2)) => n2.val + carried,
                (Some(n1), None) => n1.val + carried,
                (None, None) => carried,
            };

            carried = sum / 10;

            if sum >= 10 {
                sum %= 10;
            }

            let new_node = ListNode::new(sum);
            current.next = Some(Box::new(new_node));
            current = &mut *current.next.as_mut().unwrap();

            l1 = if l1.is_some() { l1.unwrap().next } else { l1 };
            l2 = if l2.is_some() { l2.unwrap().next } else { l2 };
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

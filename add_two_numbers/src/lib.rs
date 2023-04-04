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
        let (len1, tail1) = Self::reverse(l1);
        let (len2, tail2) = Self::reverse(l2);
        println!("{:?}", tail2);

        let (bigger, mut smaller) = if len1 > len2 {
            (tail1, tail2)
        } else {
            (tail2, tail1)
        };
        let mut carrying = false;
        let mut curr = bigger;
        let mut prev = None;

        while smaller.is_some() {
            match(curr, smaller) {
                (Some(mut big_node), Some(small_node)) => {
                    let mut total = big_node.val + small_node.val;
                    if carrying {
                        total += 1;
                    }
                    if total >= 10 {
                        total -= 10;
                        carrying = true;
                    } else {
                        carrying = false;
                    }

                    big_node.val = total;

                    let temp = big_node.next;
                    big_node.next = prev;
                    prev = Some(big_node);
                    curr = temp;

                    smaller = small_node.next;
                },
                _ => unreachable!(),
            }
        }

        // finish un-reversing the longer list
        while let Some(mut node) = curr {
            let temp = node.next;
            node.next = prev;
            if carrying {
                if node.val + 1 == 10 {
                    node.val = 0;
                } else {
                    node.val += 1;
                    carrying = false;
                }
            }
            prev = Some(node);
            curr = temp;
        }

        if carrying {
            let mut node = ListNode::new(1);
            node.next = prev;
            Some(Box::new(node))
        } else {
            prev
        }
    }

    fn reverse(l1: Option<Box<ListNode>>) -> (usize, Option<Box<ListNode>>) {
        let mut count = 0;
        let mut curr = l1;
        let mut prev = None;
        while let Some(mut node) = curr {
            let temp = node.next;
            node.next = prev;
            prev = Some(node);
            curr = temp;
            count += 1;
        }
        (count, prev)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn first() {
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

    #[test]
    fn second() {
        let v1 = vec![5];
        let v2 = vec![5];

        let mut head = Solution::add_two_numbers(create_ll(&v1), create_ll(&v2));
        let mut output = vec![];
        while let Some(node) = head {
            output.push(node.val);
            head = node.next;
        }

        assert_eq!(output, vec![1,0]);
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

use utils::{ListNode};

struct Solution;

impl Solution {
    fn reverse_list(head: ListNode) -> ListNode {
        let mut curr = Some(Box::new(head));
        let mut prev = None;

        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;

            if next.is_some() {
                prev = Some(node);
                curr = next;
            } else {
                return *node;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use utils::ListNode;

    use crate::Solution;

    #[test]
    fn it_works() {
        let input = vec![1,2,3,4,5];
        let mut head = ListNode::from(input);
        head = Solution::reverse_list(head);

        let result = 5;
        assert_eq!(result, head.val);
    }
}

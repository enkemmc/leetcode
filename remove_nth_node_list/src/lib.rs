use utils::ListNode;
struct Solution;
impl Solution{
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut ph = ListNode::new(0);
        
        let mut count = 0;
        let mut ptr = &head;
        while let Some(node) = ptr {
            count += 1;
            ptr = &node.next;
        }

        let target_index = count - n;

        ph.next = head;
        let mut ph = Some(Box::new(ph));
        let mut i = -1;
        let mut curr = ph.as_mut();

        while let Some(node) = curr {
            if i == target_index - 1 {
                node.next = if let Some(next) = node.next.take() {
                    next.next
                } else {
                    None
                };
                break;
            } else{
                curr = node.next.as_mut();
                i += 1;
            }
        }

        ph.take().unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use utils::ListNode;
    use super::Solution;

    #[test]
    fn test1() {
        let head = ListNode::from_vec(vec![1,2,3,4,5]);
        let n= 5;
        let expected = vec![2,3,4,5];
        let ans_tree = Solution::remove_nth_from_end(Some(Box::new(head)), n);
        let ans = ans_tree.unwrap().to_vec();

        assert_eq!(ans, expected);
    }
}

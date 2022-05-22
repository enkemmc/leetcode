use utils::ListNode;

impl Solution {
    pub fn merge_two_lists_2(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut curr = &mut head.next;

        let mut l = list1;
        let mut r = list2;

        while l.is_some() || r.is_some() {
            if l.is_none() {
                *curr = r;
                break;
            } else if r.is_none() {
                *curr =  l;
                break;
            } else {
                let mut templ = l.unwrap();
                let mut tempr = r.unwrap();
                if templ.val < tempr.val {
                    let next = templ.next.take();
                    l = next;
                    r = Some(tempr);
                    *curr = Some(templ);
                    curr = &mut curr.as_mut().unwrap().next;
                    
                } else {
                    let next = tempr.next.take();
                    r = next;
                    l = Some(templ);
                    *curr = Some(tempr);
                    curr = &mut curr.as_mut().unwrap().next;
                }
            }
        }
        
        head.next
    }


    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            _ => None,
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = ListNode::from(vec![1, 2, 3]);
        let wrapped = Some(Box::new(input));
        let mut ptr = wrapped;
        let mut sum = 0;
        while let Some(node) = ptr {
            sum += node.val;
            ptr = node.next;
        }
        let expected = 6;
        assert_eq!(expected, sum);
    }

    #[test]
    fn walk_through(){
        let list1 = Some(Box::new(ListNode::from(vec![1, 2, 3])));
        let list2 = Some(Box::new(ListNode::from(vec![1, 3, 4])));
        let ans_head = Solution::merge_two_lists(list1, list2);
        let mut ptr = &ans_head;
        let mut ans = vec![];
        while let Some(node) = ptr {
            ans.push(node.val);
            ptr = &node.next;
        }

        assert_eq!(vec![1, 1, 2, 3, 3, 4], ans);
    }
}

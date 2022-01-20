use utils::ListNode;
use std::mem::swap;
struct Solution;
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut focus = &mut head;
        
        while focus.is_some() {
            let first = focus.as_mut().unwrap();
            if first.next.is_some() {
                let mut second = first.next.take().unwrap();
                let third = second.next.take();
                let mut first = focus.replace(second);
                first.as_mut().unwrap().next = third;
                focus.as_mut().unwrap().next = first;
                focus = &mut focus.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                break;
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works() {
        let input_vec = vec![1,2,3,4];
        let input = ListNode::from(input_vec);
        let expected = vec![2,1,4,3];
        let ans = Solution::swap_pairs(Some(Box::new(input)));
        let readable = ans.unwrap().to_vec();
        assert_eq!(readable, expected);
    }

    #[test]
    fn experiment(){
        let input_vec = vec![1,2,3,4];
        let input = ListNode::from(input_vec);
        let expected = vec![2,1,4,3];
        let ans = Solution::swap_pairs(Some(Box::new(input)));
        let readable = ans.unwrap().to_vec();
        assert_eq!(readable, expected);
    }

    #[test]
    fn one(){
        let input_vec = vec![1];
        let input = ListNode::from(input_vec);
        let expected = vec![1];
        let ans = Solution::swap_pairs(Some(Box::new(input)));
        let readable = ans.unwrap().to_vec();
        assert_eq!(readable, expected);
    }
}

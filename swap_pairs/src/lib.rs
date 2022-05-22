use utils::ListNode;
use std::mem::swap;
struct Solution;
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut node| match node.next {
            Some(mut second) => {
                let third = Solution::swap_pairs(second.next.take());
                node.next = third;
                head = Some(second);
                head
            },
            None => head
        })
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

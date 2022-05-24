// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut v = vec![self.val];
        let mut ptr = &self.next;
        while let Some(node) = ptr {
            v.push(node.val);
            ptr = &node.next;
        }

        v
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

pub struct TreeNode<T> {
    val: T,
    left: Option<Box<T>>,
    right: Option<Box<T>>,
}

impl<T> From<Vec<T>> for TreeNode<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            val: (),
            left: (),
            right: (),
        }
    }
}

mod tests {
    #[test]
    fn first() {
        assert_eq!(true, true);
    }
}

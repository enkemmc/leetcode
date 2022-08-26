struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl <T>ListNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: None,
        }
    }

    // 0 -> 1 -> 3 -> 
    fn rev(self) -> Self {
        let mut prev = None;
        let mut curr = Some(Box::new(self));
        let mut next;

        while let Some(mut node) = curr {
            next = node.next.take();
            node.next = prev;
            prev = Some(Box::new(*node));
            curr = next;
        }

        *prev.unwrap()
    }
}

impl <T>From<Vec<T>> for ListNode<T> where T: Default {
    fn from(v: Vec<T>) -> Self {
        let mut dummy = ListNode::new(Default::default());
        let mut curr = &mut dummy.next;

        for num in v {
            *curr = Some(Box::new(ListNode::new(num)));
            curr = &mut curr.as_mut().unwrap().next;
        }

        *dummy.next.unwrap()
    }
}

fn main(){
    let input = vec![1,2,3];
    let root: ListNode<usize> = input.into();
    let head = root.rev();
    let mut curr = Some(Box::new(head));

    while let Some(node) = curr {
        println!("{}", node.val);
        curr = node.next;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn into_vec() {
        let input = vec![1,2,3];
        let root: ListNode<i32> = input.clone().into();
        let root = Some(Box::new(root));
        let mut curr = root;
        let mut ans = vec![];
        while let Some(mut node) = curr {
            curr = node.next.take();
            ans.push(node.val);
        }
        assert_eq!(ans, input);
    }

    #[test]
    fn should_fail() {
        let input = vec![1,2,3];
        let root: ListNode<i32> = input.clone().into();
        let root = Some(Box::new(root));
        let mut curr = root;
        let mut ans = vec![9];
        while let Some(mut node) = curr {
            curr = node.next.take();
            ans.push(node.val);
        }
        assert_ne!(ans, input);
    }

    #[test]
    fn rev_vec() {
        let mut input = vec![1,2,3];
        let root: ListNode<i32> = input.clone().into();
        let root = root.rev();
        input.reverse();
        let root = Some(Box::new(root));
        let mut curr = root;
        let mut ans = vec![];
        while let Some(mut node) = curr {
            curr = node.next.take();
            ans.push(node.val);
        }
        assert_eq!(ans, input);
    }
}

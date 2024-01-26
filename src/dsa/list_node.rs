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
    // Function to create a ListNode from a vector
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {ListNode::from_vec(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {ListNode::from_vec(vec![$($e.to_owned()), *])};
}

use std::mem::replace;

use crate::dsa::ListNode;

// iterative
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let mut next = node.next.take();
        node.next = prev.take();
        prev = Some(node);
        curr = next.take();
    }
    prev
}

// recursive
pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    reverse(head, None)
}

fn reverse(curr: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match curr {
        Some(mut node) => reverse(replace(&mut node.next, prev), Some(node)),
        None => prev,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked;
    use test_case::test_case;

    #[test_case(linked![1,2,3,4,5], linked![5,4,3,2,1]; "case 1")]
    #[test_case(linked![1,2], linked![2,1]; "case 2")]
    fn test_reverse_list(head: Option<Box<ListNode>>, expected: Option<Box<ListNode>>) {
        let result = reverse_list(head);
        assert_eq!(result, expected);
    }
    #[test_case(linked![1,2,3,4,5], linked![5,4,3,2,1]; "case 1")]
    #[test_case(linked![1,2], linked![2,1]; "case 2")]
    fn test_reverse_list2(head: Option<Box<ListNode>>, expected: Option<Box<ListNode>>) {
        let result = reverse_list2(head);
        assert_eq!(result, expected);
    }
}

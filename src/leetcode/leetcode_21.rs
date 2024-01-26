use crate::dsa::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut start = Box::new(ListNode::new(i32::MIN));
    let mut end = &mut start;
    let mut l1 = list1.as_ref();
    let mut l2 = list2.as_ref();

    while let (Some(n1), Some(n2)) = (l1, l2) {
        if n1.val < n2.val {
            end.next = l1.cloned();
            l1 = n1.next.as_ref();
        } else {
            end.next = l2.cloned();
            l2 = n2.next.as_ref();
        }

        end = end.next.as_mut().unwrap();
    }

    if l1.is_some() {
        end.next = l1.cloned();
    } else if l2.is_some() {
        end.next = l2.cloned();
    }

    start.next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked;
    use test_case::test_case;

    #[test_case(linked![1,2,3], linked![1,3,4], linked![1,1,2,3,3,4]; "case 1")]
    #[test_case(linked![], linked![], linked![]; "case 2")]
    #[test_case(linked![], linked![0], linked![0]; "case 3")]
    fn test_merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
        expected: Option<Box<ListNode>>,
    ) {
        let result = merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }
}

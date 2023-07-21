use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub(crate) fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }

        let mut root = Box::new(ListNode::new(0));
        let mut carry_on = 0;

        let mut l1 = l1.unwrap();
        let mut l2 = l2.unwrap();

        let mut l1val = l1.val;
        let mut l2val = l2.val;

        let mut curr = root;
        loop {
            let val = carry_on + l1val + l2val;
            if val > 10 {
                curr.val = val % 10;
                carry_on = val / 10;
            } else {
                curr.val = val;
                carry_on = 0;
            }
            // if both don't have next, we ignore
            if l1.next.is_none() && l2.next.is_none() {
                break;
            }

            if l1.next.is_some() {
                l1 = l1.next.unwrap();
                l1val = l1.val;
            } else {
                l1val = 0;
            }

            if l2.next.is_some() {
                l2 = l2.next.unwrap();
                l2val = l2.val;
            } else {
                l2val = 0;
            }

            curr.next = Some(Box::new(ListNode::new(0)));
            curr = curr.next.unwrap();
        }

        unimplemented!()
    }
}

#[cfg(test)]
mod test {}
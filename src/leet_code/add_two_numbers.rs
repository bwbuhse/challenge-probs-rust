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
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    /// Recursive helper function
    fn add_two_numbers_rec(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        // Base case
        if l1.is_none() && l2.is_none() {
            if carry > 0 {
                return Some(Box::new(ListNode::new(carry)));
            } else {
                return None;
            }
        }

        // Cases when either l1 or l2 are None, but the other isn't
        if l1.is_none() {
            let mut lln = Box::new(ListNode::new(-1));
            let sum = l2.clone().unwrap().val + carry;

            lln.val = sum % 10;
            lln.next = add_two_numbers_rec(None, l2.unwrap().next, sum / 10);
            return Some(lln);
        } else if l2.is_none() {
            let mut lln = Box::new(ListNode::new(-1));
            let sum = l1.clone().unwrap().val + carry;

            lln.val = sum % 10;
            lln.next = add_two_numbers_rec(l1.unwrap().next, None, sum / 10);
            return Some(lln);
        }

        // General case
        let mut lln = Box::new(ListNode::new(-1));
        let sum = l1.clone().unwrap().val + l2.clone().unwrap().val + carry;

        lln.val = sum % 10;
        lln.next = add_two_numbers_rec(l1.unwrap().next, l2.unwrap().next, sum / 10);
        Some(lln)
    }

    add_two_numbers_rec(l1, l2, 0)
}

// let mut x = 0;
// let mut y = 0;
// let mut cur = l1.clone();

// while let Some(n1) = cur {
//     x *= 10;
//     x += n1.val;
//     cur = n1.next;
// }

// cur = l2.clone();

// while let Some(n2) = cur {
//     y *= 10;
//     y += n2.val;
//     cur = n2.next;
// }

// Some(res)

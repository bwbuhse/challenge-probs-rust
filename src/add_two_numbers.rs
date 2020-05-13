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
    let mut x = 0;
    let mut y = 0;
    let mut cur = l1.clone();

    while let Some(n1) = cur {
        x *= 10;
        x += n1.val;
        cur = n1.next;
    }

    cur = l2.clone();

    while let Some(n2) = cur {
        y *= 10;
        y += n2.val;
        cur = n2.next;
    }

    let mut z = x + y;
    let mut prev = Box::new(ListNode::new(z % 10));
    z /= 10;

    while z != 0 {
        let temp = Box::new(ListNode::new(z % 10));
        prev.next = Some(temp.clone());
        z /= 10;
        prev = temp.clone();
    }

    while prev.

    Some(res)
}

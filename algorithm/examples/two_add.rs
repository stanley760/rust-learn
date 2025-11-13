fn main() {
    // Test case 1: [2,4,3] + [5,6,4] = [7,0,8]
    let mut l1 = Some(Box::new(ListNode::new(2)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    l1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    let mut l2 = Some(Box::new(ListNode::new(5)));
    l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
    l2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    println!(
        "Test case 1 result: {:?}",
        LinkListSolution::two_add(l1.clone(), l2.clone())
    );
    println!(
        "Test case 1 result: {:?}",
        LinkListSolution::add_two_numbers(l1, l2)
    );

    // Test case 2: [0] + [0] = [0]
    let l3 = Some(Box::new(ListNode::new(0)));
    let l4 = Some(Box::new(ListNode::new(0)));
    println!(
        "Test case 2 result: {:?}",
        LinkListSolution::two_add(l3, l4)
    );

    // Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
    let mut l5 = Some(Box::new(ListNode::new(9)));
    let mut current = &mut l5;
    for _ in 0..6 {
        current.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        current = &mut current.as_mut().unwrap().next;
    }

    let mut l6 = Some(Box::new(ListNode::new(9)));
    let mut current = &mut l6;
    for _ in 0..3 {
        current.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        current = &mut current.as_mut().unwrap().next;
    }

    println!(
        "Test case 3 result: {:?}",
        LinkListSolution::two_add(l5, l6)
    );
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct LinkListSolution;

// 使用一个 while 循环迭代判断 a 不为空或 b 不为空时进行相加操作，直到一个先为空或两个一起为空为止表示相加结束
impl LinkListSolution {
    pub(crate) fn two_add(
        mut a: Option<Box<ListNode>>,
        mut b: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut dummy;
        let mut sum = 0;
        while a.is_some() || b.is_some() || sum != 0 {
            if let Some(node) = a {
                sum += node.val;
                a = node.next;
            }
            if let Some(node) = b {
                sum += node.val;
                b = node.next;
            }
            cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            cur = &mut cur.as_mut().unwrap().next;
            sum /= 10;
        }
        dummy.unwrap().next.take()
    }

    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let l1_val = if l1.is_some() {
                l1.as_mut().unwrap().val
            } else {
                0
            };
            let l2_val = if l2.is_some() {
                l2.as_mut().unwrap().val
            } else {
                0
            };
            let sum = l1_val + l2_val + carry;
            carry = sum / 10;
            cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            cur = &mut cur.as_mut().unwrap().next;
            if l1.is_some() {
                l1 = l1.unwrap().next;
            }
            if l2.is_some() {
                l2 = l2.unwrap().next;
            }
        }
        dummy.unwrap().next.take()
    }
}

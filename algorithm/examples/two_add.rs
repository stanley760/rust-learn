fn main() {
    // Test case 1: [2,4,3] + [5,6,4] = [7,0,8]
    let mut l1 = Some(Box::new(ListNode::new(2)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    l1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    let mut l2 = Some(Box::new(ListNode::new(5)));
    l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
    l2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    println!("Test case 1 result: {:?}", LinkListSolution::two_add(l1, l2));

    // Test case 2: [0] + [0] = [0]
    let l3 = Some(Box::new(ListNode::new(0)));
    let l4 = Some(Box::new(ListNode::new(0)));
    println!("Test case 2 result: {:?}", LinkListSolution::two_add(l3, l4));

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

    println!("Test case 3 result: {:?}", LinkListSolution::two_add(l5, l6));
}



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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


impl LinkListSolution {
    pub(crate) fn two_add(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) 
        ->  Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut dummy;
        let mut sum = 0;
        while a.is_some() || b.is_some() ||  sum !=0 {
            if let Some(node) = a {
                sum += node.val;
                a = node.next;
            }
            if let Some(node) = b {
                sum += node.val;
                b = node.next;
            }
            cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum %10)));
            cur = &mut cur.as_mut().unwrap().next;
            sum /= 10;
        }
        dummy.unwrap().next.take()
    } 
}


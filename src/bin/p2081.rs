#[allow(dead_code)]
mod solution {
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    pub fn main(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // we skip the preceeding zero node.
        let mut curr = &head.unwrap().next;

        let mut output = None;
        let mut output_tail = &mut output;
        let mut sum = 0;
        while let Some(ref node) = curr {
            let &ListNode { val, ref next } = &**node;
            if val == 0 {
                *output_tail = Some(Box::new(ListNode::new(sum)));
                output_tail = &mut output_tail.as_mut().unwrap().next;
                sum = 0;
            } else {
                sum += val;
            }
            curr = next;
        }
        output
    }
}

fn main() {}

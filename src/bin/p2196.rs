mod solution {
    use std::{
        rc::Rc,
        cell::RefCell,
    };

    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }
    }

    pub fn main(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;
        type NodeMap = HashMap::<i32, (Rc<RefCell<TreeNode>>, bool)>;

        fn get_or_insert_node(nodes: &mut NodeMap, val: i32) -> (Rc<RefCell<TreeNode>>, &mut bool) {
            let &mut(ref node, ref mut has_parent) = nodes
                .entry(val)
                .or_insert_with(|| (Rc::new(RefCell::new(TreeNode::new(val))), false));
            (node.clone(), has_parent)
        }

        let mut nodes = HashMap::<i32, (Rc<RefCell<TreeNode>>, bool)>::new();
        for desc in descriptions {
            let parent_val = desc[0];
            let my_val = desc[1];
            let is_left_child = desc[2] != 0;

            let (parent, _) = get_or_insert_node(&mut nodes, parent_val);
            let (node, has_parent) = get_or_insert_node(&mut nodes, my_val);
            if is_left_child {
                parent.borrow_mut().left = Some(node.clone());
            } else {
                parent.borrow_mut().right = Some(node.clone());
            }
            *has_parent = true;
        }

        assert!(nodes.values()
            .filter(|(_, has_parent)| !has_parent)
            .count() <= 1);
        nodes.into_values()
            .filter(|(_, has_parent)| !has_parent)
            .map(|(node, _)| node)
            .next()
    }
}

fn main() {
    let _ = solution::main(vec![
        vec![20,15,1],
        vec![20,17,0],
        vec![50,20,1],
        vec![50,80,0],
        vec![80,19,1],
    ]);
}

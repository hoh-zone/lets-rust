use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub val: u32,
    pub neighbors: RefCell<Vec<Weak<Node>>>,
}

pub fn make() -> Vec<Rc<Node>> {
    let node0 = Rc::new(Node {
        val: 0,
        neighbors: RefCell::new(vec![]),
    });
    let node1 = Rc::new(Node {
        val: 1,
        neighbors: RefCell::new(vec![]),
    });
    let node2 = Rc::new(Node {
        val: 2,
        neighbors: RefCell::new(vec![]),
    });
    
    node0.neighbors.borrow_mut().push(Rc::downgrade(&node1));
    node1.neighbors.borrow_mut().push(Rc::downgrade(&node2));
    node2.neighbors.borrow_mut().push(Rc::downgrade(&node0));
    
    vec![node0, node1, node2]
}

pub fn walk(mut nodes: Vec<Rc<Node>>) -> Vec<u32> {
    let mut queue = vec![nodes.pop().unwrap()];
    let mut vals: Vec<u32> = vec![];

    while let Some(node) = queue.pop() {
        vals.push(node.val);
        let neighbors: Ref<'_, Vec<Weak<Node>>> = node.neighbors.borrow();
        for weak_ref in &*neighbors {
            if let Some(node_to_visit) = weak_ref.upgrade() {
                queue.push(node_to_visit);
            }
        }
        // node is dropped, so all weak references to this node
        // will return None when upgrade() is called
    }

    vals
}
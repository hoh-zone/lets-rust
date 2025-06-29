use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub val: u32,
    pub neighbors: RefCell<Vec<Rc<Node>>>,
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
    
    node0.neighbors.borrow_mut().push(Rc::clone(&node1));
    
    node0.neighbors.borrow_mut().push(Rc::clone(&node2));
    
    node1.neighbors.borrow_mut().push(Rc::clone(&node2));
    
    vec![node0, node1, node2]
}
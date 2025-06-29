use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub val: u32,
    pub neighbors: Vec<Rc<Node>>,
}

pub fn make() -> Vec<Rc<Node>> {
    let node0 = Rc::new(Node {
        val: 0,
        neighbors: vec![],
    });
    let node1 = Rc::new(Node {
        val: 1,
        neighbors: vec![Rc::clone(&node0)],
    });
    let node2 = Rc::new(Node {
        val: 2,
        neighbors: vec![Rc::clone(&node0)],
    });
    
    vec![node0, node1, node2]
}
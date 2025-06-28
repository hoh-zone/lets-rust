pub fn wrap(val: u32) -> Box<u32> {
    Box::new(val)
}

pub fn unwrap(x: Box<u32>) -> u32 {
    *x
}

#[derive(Debug)]
pub struct Tree {
    pub val: i32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

pub fn make_tree() -> Tree {
    Tree {
        val: 1,
        left:Some(Box::new(Tree {
            val: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Tree {
            val: 3,
            left: None,
            right: None,
        })),
    }
}
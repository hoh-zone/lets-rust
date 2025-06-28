#[derive(Debug)]
pub struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}
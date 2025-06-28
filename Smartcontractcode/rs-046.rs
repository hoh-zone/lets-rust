pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct VecIter<T> {
    next_index: usize,
    vec: Vec<T>,
}

impl<T> VecIter<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self {
            next_index: 0,
            vec: v,
        }
    }
}

impl<T: Copy> Iterator for VecIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.vec.get(self.next_index) {
            self.next_index += 1;
            Some(*val)
        } else {
            None
        }
        
    }
}
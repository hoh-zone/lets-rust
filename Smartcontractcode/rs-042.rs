use std::convert::{From, Into};

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u32,
    pub hobbies: Vec<String>,
}

impl From<(&str, u32, &str)> for Cat {
    fn from(vals: (&str, u32, &str)) -> Self {
        Cat {
            name: vals.0.to_string(),
            age: vals.1,
            hobbies: vec![vals.2.to_string()],
        }
    }
}

impl Into<Cat> for (&str, u32) {
    fn into(self) -> Cat {
        Cat {
            name: self.0.to_string(),
            age: self.1,
            hobbies: vec!["sleep in a box".to_string()],
        }
    }
}

#[derive(Debug)]
pub struct Pair<A, B>(pub A, pub B);

impl<A, B> From<(A, B)> for Pair<A, B> {
    fn from(val: (A, B)) -> Self {
        Pair(val.0, val.1)
    }
}

impl Into<Pair<u32, u32>> for u32 {
    fn into(self) -> Pair<u32, u32> {
        Pair(self, self)
    }
}
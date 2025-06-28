use std::ops::AddAssign;

pub fn sum<T: Default + Copy + AddAssign>(vals: &[T]) -> T {
    let mut total = Default::default();
    for v in vals {
        total += *v;
    }
    total
}

use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<T> {
    pub vertices: HashSet<T>,
    pub edges: HashSet<(T, T)>,
}

pub fn graph_to_hash_set<T>(g: Graph<T>) -> HashMap<T, HashSet<T>>
where
    T: Hash + Eq,
{
    let mut map = HashMap::new();

    for v in g.vertices {
        map.insert(v, HashSet::new());
    }

    for e in g.edges {
        if let Some(set) = map.get_mut(&e.0) {
            set.insert(e.1);
        }
    }

    map
}
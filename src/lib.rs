use std::collections::HashMap;

#[derive(Debug)]
pub struct IndexMap<T> {
    index: usize,
    map: HashMap<T, usize>
}

impl <T> IndexMap<T> where
    T: std::cmp::Eq + std::hash::Hash
{
    pub fn with_capacity(capacity: usize) -> IndexMap<T> {
        IndexMap {
            index: 0, map: HashMap::with_capacity(capacity)
        }
    }

    pub fn new() -> IndexMap<T> {
        IndexMap { index: 0, map: HashMap::new() }
    }

    pub fn insert(&mut self, key: T) -> usize {
        if let Some(ind) = self.map.get(&key) {
            *ind
        } else {
            let idx = self.index;
            self.map.insert(key, idx);
            self.index += 1;
            idx
        }
    }

    pub fn get(&self, key: &T) -> Option<&usize> {
        self.map.get(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

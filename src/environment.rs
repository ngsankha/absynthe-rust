use std::collections::{hash_map, HashMap};

pub struct Environment<T> {
    map: HashMap<String, T>,
}

impl<T> Environment<T> {
    pub fn new() -> Environment<T> {
        Environment {
            map: HashMap::new(),
        }
    }

    pub fn put(&mut self, var: String, val: T) {
        self.map.insert(var, val);
    }

    pub fn get(&self, var: String) -> Option<&T> {
        self.map.get(&var)
    }

    pub fn keys(&self) -> hash_map::Keys<String, T> {
        self.map.keys()
    }

    pub fn iter(&self) -> hash_map::Iter<String, T> {
        self.map.iter()
    }
}

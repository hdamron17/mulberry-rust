use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use super::group::Group;

#[derive(Clone, Debug)]
pub struct Tree<K, T> {
    keys: HashSet<K>,
    parents: HashMap<K, K>,
    transforms: HashMap<(K, K), T>,
}

impl<K, T> Default for Tree<K, T> {
    fn default() -> Self {
        Tree {
            keys: HashSet::default(),
            parents: HashMap::default(),
            transforms: HashMap::default(),
        }
    }
}

impl<K, T> Tree<K, T> {
    pub fn new() -> Self {
        Tree::default()
    }
}

impl<K: Eq + Hash + Clone, T: Group + Clone> Tree<K, T> {
    pub fn set_transform(&mut self, transform: T, from_key: K, to_key: K) {
        let from_to_key = (from_key, to_key);
        if self.transforms.contains_key(&from_to_key) {
            // Replace current transformation
            self.transforms.remove(&from_to_key); // Delete other direction also
            self.transforms.insert(from_to_key, transform);
        } else {
            let (from_key, to_key) = from_to_key;
            let has_from_key = self.parents.contains_key(&from_key);
            let has_to_key = self.parents.contains_key(&to_key);
            if has_from_key && has_to_key {
                todo!("Check not same tree then connect");
            } else {
                self.transforms
                    .insert((from_key.clone(), to_key.clone()), transform);

                let (from_key, to_key) = if has_to_key {
                    (to_key, from_key) // Reverse order since `to_key` already has parent
                } else {
                    (from_key, to_key)
                };

                self.keys.insert(to_key.clone());
                self.parents.insert(to_key, from_key);
            }
        }
    }

    pub fn get_direct_transform(&self, from_key: K, to_key: K) -> Option<T> {
        let from_to_key = (from_key, to_key);
        self.transforms.get(&from_to_key).map(T::clone).or_else(|| {
            let to_from_key = (from_to_key.1, from_to_key.0);
            self.transforms.get(&to_from_key).map(T::inverse)
        })
    }
}

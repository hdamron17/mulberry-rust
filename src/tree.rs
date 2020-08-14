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
    pub fn has_frame(&self, key: &K) -> bool {
        self.keys.contains(key)
    }

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

    fn get_ancestry(&self, mut key: K) -> Vec<K> {
        let mut ret: Vec<_> = std::iter::from_fn(|| {
            self.parents.get(&key).map(|newkey| {
                let mut newkey = newkey.clone();
                std::mem::swap(&mut key, &mut newkey);
                newkey
            })
        })
        .collect();
        ret.push(key); // Add last key
        ret
    }

    fn get_root(&self, key: K) -> K {
        self.get_ancestry(key).last().unwrap().clone() // One element guaranteed
    }

    pub fn can_transform(&self, from_key: K, to_key: K) -> bool {
        self.get_root(from_key) == self.get_root(to_key)
    }

    pub fn get_path(&self, from_key: K, to_key: K) -> Option<Vec<K>> {
        if from_key == to_key {
            return Some(vec![from_key]);
        }

        let mut from_root_path = self.get_ancestry(from_key).into_iter().rev().peekable();
        let mut to_root_path = self.get_ancestry(to_key).into_iter().rev().peekable();
        let mut center = None; // Connecting frame

        // The paths cannot be the same since `from_key` != `to_key`, hence no infinite loop
        while from_root_path.peek() == to_root_path.peek() {
            center = from_root_path.next();
            to_root_path.next();
        }

        Some(
            from_root_path
                .rev() // Go up `from_key` up to `center`
                .chain(std::iter::once(center?)) // Include `center` if there was a connection
                .chain(to_root_path) // `Go down `center` to `to_key`
                .collect(),
        )
    }

    fn get_direct_transform(&self, from_key: K, to_key: K) -> Option<T> {
        let from_to_key = (from_key, to_key);
        self.transforms.get(&from_to_key).map(T::clone).or_else(|| {
            let to_from_key = (from_to_key.1, from_to_key.0); // Try the other direction
            self.transforms.get(&to_from_key).map(T::inverse)
        })
    }

    fn get_transform_from_path(&self, path: Vec<K>) -> T {
        let (head, tail) = match path.split_first() {
            Some((head, tail)) => (head, tail),
            None => return T::identity(),
        };
        tail.into_iter()
            .fold((T::identity(), head), |(acc, prevkey), key| {
                let transform = self
                    .get_direct_transform(prevkey.clone(), key.clone())
                    .expect("Invalid path"); // TODO return a recoverable error
                (acc.compose(&transform), key)
            })
            .0
    }

    pub fn get_transform(&self, from_key: K, to_key: K) -> Option<T> {
        let path = self.get_path(from_key, to_key)?;
        Some(self.get_transform_from_path(path))
    }
}

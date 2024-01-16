use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Dictionary<K, V> {
    arr: [Option<(K, V)>; 500],
}

impl<K: Copy + Hash, V: Copy> Dictionary<K, V> {
    pub fn new() -> Dictionary<K, V> {
        Dictionary { arr: [None; 500] }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let key_value = (key, value);

        let index = Self::hash_key(key) % self.arr.len() as u64;

        self.arr[index as usize] = Some(key_value);
    }

    pub fn get(&self, key: K) -> Option<V> {
        let index = Self::hash_key(key) % self.arr.len() as u64;

        match self.arr[index as usize] {
            Some(key_value) => Some(key_value.1),
            None => None,
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let index = Self::hash_key(key) % self.arr.len() as u64;

        let removed_value = self.arr[index as usize];
        self.arr[index as usize] = None;

        match removed_value {
            Some(key_value) => Some(key_value.1),
            None => None,
        }
    }

    fn hash_key(key: K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        hasher.finish()
    }
}

impl<K: Copy + Hash, V: Copy> Default for Dictionary<K, V> {
    fn default() -> Dictionary<K, V> {
        Self::new()
    }
}

// https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b
// https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html

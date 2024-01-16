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

        let index = Self::hash_key(key) % self.len() as u64;

        self.arr[index as usize] = Some(key_value);
    }

    pub fn get(&self, key: K) -> Option<V> {
        let index = Self::hash_key(key) % self.len() as u64;

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

    fn len(&self) -> usize {
        self.arr.len()
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

#[test]
fn assert_hashmap_get() {
    let mut dict: Dictionary<&str, u32> = Dictionary::new();

    dict.insert("banana", 10);

    assert_eq!(dict.get("banana"), Some(10));
    assert_eq!(dict.get("laranja"), None);
}

#[test]
fn assert_hashmap_remove() {
    let mut dict: Dictionary<&str, u32> = Dictionary::new();

    dict.insert("banana", 10);

    assert_eq!(dict.remove("banana"), Some(10));
    assert_eq!(dict.remove("laranja"), None);
}

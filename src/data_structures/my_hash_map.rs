use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct MyHashMap<K, V> {
    arr: [Option<(K, V)>; 600],
}

impl<K: Copy + Hash, V: Copy> MyHashMap<K, V> {
    pub fn new() -> MyHashMap<K, V> {
        MyHashMap { arr: [None; 600] }
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

impl<K: Copy + Hash, V: Copy> Default for MyHashMap<K, V> {
    fn default() -> MyHashMap<K, V> {
        Self::new()
    }
}

#[test]
fn assert_hashmap_get_item() {
    let mut dict: MyHashMap<&str, u32> = MyHashMap::new();

    dict.insert("banana", 10);

    assert_eq!(dict.get("banana"), Some(10));
    assert_eq!(dict.get("orange"), None);
}

#[test]
fn assert_hashmap_remove_item() {
    let mut dict: MyHashMap<&str, u32> = MyHashMap::new();

    dict.insert("banana", 10);

    assert_eq!(dict.remove("banana"), Some(10));
    assert_eq!(dict.remove("orange"), None);
}

#[test]
fn assert_hashmap_update_item() {
    let mut dict: MyHashMap<&str, u8> = MyHashMap::new();

    dict.insert("watermelon", 3);
    dict.insert("watermelon", 18);

    assert_eq!(dict.get("watermelon"), Some(18));
}

#[test]
fn assert_hashmap_insert_multiple_items() {
    let mut dict: MyHashMap<&str, &str> = MyHashMap::new();

    dict.insert("watermelon", "red");
    dict.insert("potato", "yellow");
    dict.insert("orange", "orange");
    dict.insert("grape", "purple");

    assert_eq!(dict.get("watermelon"), Some("red"));
    assert_eq!(dict.get("potato"), Some("yellow"));
    assert_eq!(dict.get("orange"), Some("orange"));
    assert_eq!(dict.get("grape"), Some("purple"));
}

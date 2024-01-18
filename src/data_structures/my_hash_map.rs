use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct MyHashMap<K, V> {
    vec: Vec<Option<(K, V)>>,
}

impl<K: Clone + Hash, V: Clone> MyHashMap<K, V> {
    pub fn new() -> MyHashMap<K, V> {
        MyHashMap {
            vec: vec![None; 500],
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = Self::hash_key(&key) % self.len() as u64;

        self.vec[index as usize] = Some((key, value.clone()));

        Some(value)
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let index = Self::hash_key(&key) % self.len() as u64;

        match self.vec.get(index as usize) {
            Some(Some((_key, value))) => Some(value),
            _ => None,
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let index = Self::hash_key(&key) % self.vec.len() as u64;

        let removed_value = self.vec.remove(index as usize);
        self.vec.insert(index as usize, None);

        match removed_value {
            Some(key_value) => Some(key_value.1),
            None => None,
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn hash_key(key: &K) -> u64 {
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
    let mut dict: MyHashMap<String, u32> = MyHashMap::new();

    dict.insert(String::from("banana"), 7880);

    assert_eq!(dict.get(String::from("banana")), Some(&7880));
    assert_eq!(dict.get(String::from("orange")), None);
}

#[test]
fn assert_hashmap_remove_item() {
    let mut dict: MyHashMap<String, u32> = MyHashMap::new();

    dict.insert("banana".to_string(), 10);

    assert_eq!(dict.remove(String::from("banana")), Some(10));
    assert_eq!(dict.remove(String::from("banana")), None);
}

#[test]
fn assert_hashmap_update_item() {
    let mut dict: MyHashMap<String, u32> = MyHashMap::new();

    dict.insert("watermelon".to_string(), 3);
    dict.insert("watermelon".to_string(), 18);

    assert_eq!(dict.get(String::from("watermelon")), Some(&18));
}

#[test]
fn assert_hashmap_get_multiple_key_value_types() {
    let mut dict1: MyHashMap<String, u8> = MyHashMap::new();
    let mut dict2: MyHashMap<u64, &str> = MyHashMap::new();
    let mut dict3: MyHashMap<u8, String> = MyHashMap::new();
    let mut dict4: MyHashMap<&str, u32> = MyHashMap::new();

    dict1.insert(String::from("banana"), 17);
    dict2.insert(778, "plane");
    dict3.insert(10, String::from("lover"));
    dict4.insert("watermelon", 1082);

    assert_eq!(dict1.get(String::from("banana")), Some(&17));
    assert_eq!(dict2.get(778), Some(&"plane"));
    assert_eq!(dict3.get(10u8), Some(&String::from("lover")));
    assert_eq!(dict4.get("watermelon"), Some(&1082));
}

#[test]
fn assert_hashmap_remove_multiple_key_value_types() {
    let mut dict1: MyHashMap<String, u8> = MyHashMap::new();
    let mut dict2: MyHashMap<u64, &str> = MyHashMap::new();
    let mut dict3: MyHashMap<u8, String> = MyHashMap::new();
    let mut dict4: MyHashMap<&str, u32> = MyHashMap::new();

    dict1.insert(String::from("banana"), 17);
    dict2.insert(778, "plane");
    dict3.insert(10, String::from("lover"));
    dict4.insert("watermelon", 1082);

    assert_eq!(dict1.remove(String::from("banana")), Some(17));
    assert_eq!(dict2.remove(778), Some("plane"));
    assert_eq!(dict3.remove(10u8), Some(String::from("lover")));
    assert_eq!(dict4.remove("watermelon"), Some(1082));
}

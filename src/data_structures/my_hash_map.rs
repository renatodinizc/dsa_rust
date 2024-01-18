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
fn insert_and_get() {
    let mut map = MyHashMap::new();
    assert_eq!(map.insert("key1", 10), Some(10));
    assert_eq!(map.get("key1"), Some(&10));
    assert_eq!(map.get("key2"), None);
}

#[test]
fn overwrite_value() {
    let mut map = MyHashMap::new();
    map.insert("key1", 10);
    assert_eq!(map.insert("key1", 20), Some(20));
    assert_eq!(map.get("key1"), Some(&20));
}

#[test]
fn remove_key() {
    let mut map = MyHashMap::new();
    map.insert("key1", 10);
    assert_eq!(map.remove("key1"), Some(10));
    assert_eq!(map.get("key1"), None);
}

#[test]
fn remove_nonexistent_key() {
    let mut map: MyHashMap<&str, u32> = MyHashMap::new();
    assert_eq!(map.remove("key1"), None);
}

#[test]
fn handling_collisions() {
    let mut map = MyHashMap::new();
    map.insert(1, "value1");
    map.insert(501, "value2"); // Assuming it will collide with 1

    assert_eq!(map.get(1), Some(&"value1"));
    assert_eq!(map.get(501), Some(&"value2")); // Checks if the collision is handled
}

#[test]
fn string_keys_and_values() {
    let mut map = MyHashMap::new();
    map.insert("apple", "red");
    map.insert("banana", "yellow");

    assert_eq!(map.get("apple"), Some(&"red"));
    assert_eq!(map.get("banana"), Some(&"yellow"));
    assert_eq!(map.get("grape"), None);
}

#[test]
fn integer_keys_and_struct_values() {
    #[derive(Debug, PartialEq, Clone)]
    struct Fruit {
        name: String,
        color: String,
    }

    let mut map = MyHashMap::new();
    map.insert(
        1,
        Fruit {
            name: "apple".to_string(),
            color: "red".to_string(),
        },
    );
    map.insert(
        2,
        Fruit {
            name: "banana".to_string(),
            color: "yellow".to_string(),
        },
    );

    assert_eq!(
        map.get(1),
        Some(&Fruit {
            name: "apple".to_string(),
            color: "red".to_string()
        })
    );
    assert_eq!(
        map.get(2),
        Some(&Fruit {
            name: "banana".to_string(),
            color: "yellow".to_string()
        })
    );
    assert_eq!(map.get(3), None);
}

#[test]
fn complex_key_structures() {
    #[derive(Hash, PartialEq, Eq, Clone)]
    struct ComplexKey {
        part1: i32,
        part2: String,
    }

    let mut map = MyHashMap::new();
    let key1 = ComplexKey {
        part1: 1,
        part2: "one".to_string(),
    };
    let key2 = ComplexKey {
        part1: 2,
        part2: "two".to_string(),
    };

    map.insert(key1.clone(), "Value1");
    map.insert(key2.clone(), "Value2");

    assert_eq!(map.get(key1), Some(&"Value1"));
    assert_eq!(map.get(key2), Some(&"Value2"));
}

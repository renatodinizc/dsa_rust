use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ## Note
// Khe implementation is theoretical and may lead to poor developer experience
// and memory optimization.
// It is advisable to consider alternative data structures
// or resizing strategies for practical use.

pub struct Dictionary<K, V> {
    arr: [Option<KeyValue<K, V>>; 1000],
}

#[derive(Copy, Clone, Debug)]
pub struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
}

impl <K: Copy + Hash, V: Copy> Dictionary<K,V> {
    pub fn new() -> Dictionary<K,V> {
        Dictionary {
            arr: [None; 1000]
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let key_value = KeyValue {
            key,
            value,
        };

        let index = Dictionary::<K, V>::hash_key(key) % self.arr.len() as u64;

        self.arr[index as usize] = Some(key_value);
    }

    pub fn get(&self, key: K) -> Option<KeyValue<K, V>>{
        let index = Dictionary::<K, V>::hash_key(key) % self.arr.len() as u64;

        self.arr[index as usize]
    }

    fn hash_key(key: K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        hasher.finish()
    }
}


// KODO:
// https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b
// https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html

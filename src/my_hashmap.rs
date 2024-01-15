use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ## Note
// The implementation is theoretical and may lead to poor developer experience
// and memory optimization.
// It is advisable to consider alternative data structures
// or resizing strategies for practical use.

pub fn create_hash() -> [u32; 500] {
    let hsh = [0; 500];
    hsh
}

pub fn insert_into_my_hashmap(hsh: &mut [u32; 500], key: &str, value: u32) {
    let index_to_insert = hash_key(&key) % hsh.len();

    hsh[index_to_insert] = value;
}

pub fn get_from_my_hashmap(hsh: &[u32; 500], key: &str) -> u32 {
    let index_to_search = hash_key(&key) % hsh.len();

    hsh[index_to_search]
}

fn hash_key(key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    usize::try_from(hasher.finish()).unwrap()
}

// TODO:
// https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b
// https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html

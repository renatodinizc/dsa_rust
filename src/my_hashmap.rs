use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ## Note
// The implementation is theoretical and may lead to memory overflow
// due to excessive vector lengths.
// It is advisable to consider alternative data structures
// or resizing strategies for practical use.

pub fn create_hash() -> Vec<usize> {
    let vec: Vec<usize> = Vec::new();
    vec
}

pub fn insert_into_my_hashmap(mut hsh: Vec<usize>, key: String, value: usize) -> Vec<usize> {
    let index_to_insert = hash_key(&key);

    if index_to_insert > hsh.len() {
        hsh.resize(index_to_insert + 1, 0); // Resize and fill with default value (0 in this case)
    }
    hsh.insert(index_to_insert, value);

    hsh
}

pub fn get_from_my_hashmap(hsh: Vec<usize>, key: String) -> usize {
    let index_to_search = hash_key(&key);

    *hsh.get(index_to_search).unwrap()
}

fn hash_key(key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    usize::try_from(hasher.finish()).unwrap()
}

// TODO:
// https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b
// https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html

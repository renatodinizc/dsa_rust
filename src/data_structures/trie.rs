use std::collections::HashMap;

pub struct TrieNode {
    data: HashMap<char, Box<TrieNode>>,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode {
                data: HashMap::new(),
            },
        }
    }

    pub fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;

        for letter in word.chars() {
            match current_node.data.get(&letter) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        true
    }

    pub fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for letter in word.chars() {
            current_node = current_node.data.entry(letter).or_insert_with(|| {
                Box::new(TrieNode {
                    data: HashMap::new(),
                })
            });
        }

        current_node.data.insert(
            '*',
            Box::new(TrieNode {
                data: HashMap::new(),
            }),
        );
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_trie() {}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();

        trie.insert("hello".to_string());

        assert_eq!(trie.search("hello".to_string()), true);
        assert_eq!(trie.search("world".to_string()), false);
    }

    #[test]
    fn test_search_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());

        assert_eq!(trie.search("hell".to_string()), true);
    }

    #[test]
    fn test_insert_and_search_multiple() {
        let mut trie = Trie::new();
        trie.insert("oil".to_string());
        trie.insert("car".to_string());
        trie.insert("oiled".to_string());

        assert_eq!(trie.search("oil".to_string()), true);
        assert_eq!(trie.search("car".to_string()), true);
        assert_eq!(trie.search("oiled".to_string()), true);
        assert_eq!(trie.search("oiling".to_string()), false);
    }

    #[test]
    fn test_empty_string() {
        let mut trie = Trie::new();

        trie.insert("".to_string());

        assert_eq!(trie.search("".to_string()), true);
    }

    #[test]
    fn test_nonexistent_word() {
        let mut trie = Trie::new();
        trie.insert("exist".to_string());

        assert_eq!(trie.search("nonexistent".to_string()), false);
    }
}

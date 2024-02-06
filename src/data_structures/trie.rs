use std::collections::HashMap;

#[derive(Clone)]
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

    pub fn search(&self, word: String) -> Option<TrieNode> {
        let mut current_node = &self.root;

        for letter in word.chars() {
            match current_node.data.get(&letter) {
                Some(node) => current_node = node,
                None => return None,
            }
        }

        Some(current_node.clone())
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

    pub fn collect_all_words<'a>(
        &'a self,
        node: &TrieNode,
        word: String,
        words: &'a mut Vec<String>,
    ) -> Vec<String> {
        for (letter, child_node) in &node.data {
            if *letter == '*' {
                words.push(word.clone());
            } else {
                let mut new_word = word.clone();
                new_word.push(*letter);
                Self::collect_all_words(self, child_node, new_word, words);
            }
        }

        words.clone()
    }

    pub fn autocomplete(&self, prefix: String) -> Option<Vec<String>> {
        let mut words: Vec<String> = Vec::new();
        match Self::search(self, prefix.clone()) {
            None => None,
            Some(ref node) => Some(
                Self::collect_all_words(self, node, String::new(), &mut words)
                    .iter()
                    .map(|word| prefix.clone() + word)
                    .collect(),
            ),
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();

        trie.insert("hello".to_string());

        assert!(trie.search("hello".to_string()).is_some());
        assert!(trie.search("world".to_string()).is_none());
    }

    #[test]
    fn test_search_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());

        assert!(trie.search("hell".to_string()).is_some());
    }

    #[test]
    fn test_insert_and_search_multiple() {
        let mut trie = Trie::new();
        trie.insert("oil".to_string());
        trie.insert("car".to_string());
        trie.insert("oiled".to_string());

        assert!(trie.search("oil".to_string()).is_some());
        assert!(trie.search("car".to_string()).is_some());
        assert!(trie.search("oiled".to_string()).is_some());
        assert!(trie.search("oiling".to_string()).is_none());
    }

    #[test]
    fn test_empty_string() {
        let mut trie = Trie::new();

        trie.insert("".to_string());

        assert!(trie.search("".to_string()).is_some());
    }

    #[test]
    fn test_nonexistent_word() {
        let mut trie = Trie::new();
        trie.insert("exist".to_string());

        assert!(trie.search("nonexistent".to_string()).is_none());
    }

    #[test]
    fn collect_all_words() {
        let mut trie = Trie::new();
        trie.insert("banana".to_string());
        trie.insert("bananas".to_string());
        trie.insert("banner".to_string());
        trie.insert("strawberry".to_string());
        trie.insert("peach".to_string());
        trie.insert("grape".to_string());

        let mut words: Vec<String> = Vec::new();
        trie.collect_all_words(&trie.root, String::new(), &mut words);

        assert!(words.contains(&"banana".to_string()));
        assert!(words.contains(&"bananas".to_string()));
        assert!(words.contains(&"banner".to_string()));
        assert!(words.contains(&"strawberry".to_string()));
        assert!(words.contains(&"peach".to_string()));
        assert!(words.contains(&"grape".to_string()));
        assert_eq!(words.contains(&"not_inserted".to_string()), false);
    }

    #[test]
    fn autocompletes_word() {
        let mut trie = Trie::new();
        trie.insert("banana".to_string());
        trie.insert("bananas".to_string());
        trie.insert("banner".to_string());

        let result = trie.autocomplete("ban".to_string());

        assert!(result.is_some());
        assert!(result.clone().unwrap().contains(&"banana".to_string()));
        assert!(result.clone().unwrap().contains(&"bananas".to_string()));
        assert!(result.clone().unwrap().contains(&"banner".to_string()));
    }
}

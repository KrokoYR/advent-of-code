/// Simple trie implementation,
/// taken from this article - https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m
use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }

        current_node.is_end = true;
    }

    /// returns (contains, is_end)
    pub fn contains(&self, word: &str) -> (bool, bool) {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return (false, false),
            }
        }

        (true, current_node.is_end)
    }

    pub fn contains_chars(&self, chars: &[char]) -> (bool, bool) {
        let mut current_node = &self.root;

        for c in chars {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return (false, false),
            }
        }

        (true, current_node.is_end)
    }
}

impl From<Vec<&str>> for Trie {
    fn from(words: Vec<&str>) -> Self {
        let mut trie = Trie::new();

        for word in words {
            trie.insert(word);
        }

        trie
    }
}

#[cfg(test)]
mod trie_tests {
    use super::*;
    #[test]
    fn it_works() {
        let prefixes = Trie::from(vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ]);

        assert_eq!(prefixes.contains_chars(&vec!['o', 'n', 'e']), (true, true));
    }
}

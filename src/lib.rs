use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, usize>,
    value: Option<usize>,
}

struct SuccinctTrie {
    nodes: Vec<TrieNode>,
    prefixes: Vec<usize>,
    suffixes: Vec<char>,
}

impl SuccinctTrie {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode { children: HashMap::new(), value: None }],
            prefixes: vec![0],
            suffixes: vec![],
        }
    }

    fn insert(&mut self, key: &str, value: usize) {
        let mut node_id = 0;
        for c in key.chars() {
            let node = &mut self.nodes[node_id];
            if !node.children.contains_key(&c) {
                self.nodes.push(TrieNode { children: HashMap::new(), value: None });
                let new_node_id = self.nodes.len() - 1;
                node.children.insert(c, new_node_id);
                self.prefixes.push(self.suffixes.len());
            }
            node_id = *node.children.get(&c).unwrap();
            self.suffixes.push(c);
        }
        let node = &mut self.nodes[node_id];
        node.value = Some(value);
    }

    fn search(&self, key: &str) -> Option<usize> {
        let mut node_id = 0;
        for c in key.chars() {
            let node = &self.nodes[node_id];
            if !node.children.contains_key(&c) {
                return None;
            }
            node_id = *node.children.get(&c).unwrap();
        }
        let node = &self.nodes[node_id];
        node.value
    }
}


#[cfg(test)]
fn test_succinct_trie() {
    let mut trie = SuccinctTrie::new();

    // Insert key-value pairs into the trie
    trie.insert("apple", 0);
    trie.insert("banana", 1);
    trie.insert("cherry", 2);

    // Search for existing and non-existing keys
    assert_eq!(trie.search("apple"), Some(0));
    assert_eq!(trie.search("banana"), Some(1));
    assert_eq!(trie.search("cherry"), Some(2));
    assert_eq!(trie.search("durian"), None);

    // Insert another key-value pair and search again
    trie.insert("cranberry", 3);
    assert_eq!(trie.search("cranberry"), Some(3));
}

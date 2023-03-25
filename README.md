# Bits: A Succinct Trie Implementation

A Succinct Trie is a compressed data structure used for efficient string searching and indexing. This Rust crate provides an implementation of the Succinct Trie data structure.

## Usage

To use the `succinct-trie` crate in your Rust project, simply add it to your `Cargo.toml` file:

```toml
[dependencies]
succinct-trie = "0.1.0"
```

Then, you can create a new `SuccinctTrie` and insert key-value pairs using the `insert` method:


```rs
use succinct_trie::SuccinctTrie;

let mut trie = SuccinctTrie::new();
trie.insert("apple", 0);
trie.insert("banana", 1);
trie.insert("cherry", 2);
```

You can search for keys using the `search` method:

```rs
assert_eq!(trie.search("apple"), Some(0));
assert_eq!(trie.search("banana"), Some(1));
assert_eq!(trie.search("cherry"), Some(2));
assert_eq!(trie.search("durian"), None);
```

And you can serialize and deserialize the SuccinctTrie using the `serde` library:

```rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: usize,
}

let mut trie = SuccinctTrie::new();
trie.insert("apple", 0);
trie.insert("banana", 1);
trie.insert("cherry", 2);

let encoded = serde_json::to_string(&trie).unwrap();
let decoded: SuccinctTrie<KeyValue> = serde_json::from_str(&encoded).unwrap();

assert_eq!(decoded.search("apple"), Some(KeyValue { key: "apple".to_string(), value: 0 }));
assert_eq!(decoded.search("banana"), Some(KeyValue { key: "banana".to_string(), value: 1 }));
assert_eq!(decoded.search("cherry"), Some(KeyValue { key: "cherry".to_string(), value: 2 }));

```

## License

This crate is licensed under the MIT license. See the `LICENSE` file for more details.
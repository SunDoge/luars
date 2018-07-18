use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct DictUnit {
    word: char,
    weight: f64,
    tag: String,
}

#[derive(Debug)]
pub struct Dag {
    runestr: String,
    nexts: Vec<(usize, DictUnit)>,
    weight: f64,
    next_pos: usize,
}

#[derive(Debug)]
struct TrieNode {
    value: Option<DictUnit>,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            value: None,
            children: HashMap::new(),
        }
    }

    // pub fn from(value: DictUnit) -> TrieNode {
    //     TrieNode {
    //         value: Some(value),
    //         children: HashMap::new(),
    //     }
    // }

    pub fn insert(&mut self, key: &[char], value: DictUnit) {
        if key.is_empty() {
            match self.value {
                Some(_) => panic!("key exists"),
                None => {
                    self.value = Some(value);
                }
            }
            return;
        }

        self.children
            .entry(key[0])
            .or_insert(TrieNode::new())
            .insert(&key[1..], value);
        
    }

    pub fn find(&self, key: &[char]) -> Option<&DictUnit> {
        match key.len() {
            0 => self.value.as_ref(),
            _ => self.children.get(&key[0]).and_then(|c| c.find(&key[1..])),
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::new(),
        }
    }

    // pub fn from() -> Trie {

    // }

    pub fn insert(&mut self, key: &str, value: DictUnit) {
        if key.is_empty() {
            return;
        }

        let chars: Vec<char> = key.chars().collect();
        self.root.insert(&chars, value);
    }
}

#[cfg(test)]
mod tests {
    use trie::*;

    #[test]
    fn trie() {
        let mut t = Trie::new();
        t.insert(
            "我们",
            DictUnit {
                word: 'w',
                weight: 0.1,
                tag: "s".to_string(),
            },
        );
        t.insert(
            "我的",
            DictUnit {
                word: 'w',
                weight: 0.1,
                tag: "s".to_string(),
            },
        );

        println!("{:#?}", t);
    }
}
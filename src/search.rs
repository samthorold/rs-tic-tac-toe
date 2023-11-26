use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait Node: Sized + Clone + Hash {
    fn children(&self) -> Vec<Self>;
    fn is_terminal(&self) -> bool;
    fn score(&self) -> i32;
    fn is_maximising(&self) -> bool;
}

pub struct Search {
    scores: HashMap<u64, i32>,
}

impl Search {
    pub fn new() -> Search {
        Search {
            scores: HashMap::new(),
        }
    }

    pub fn alphabeta<T: Node>(&mut self, node: &T, mut a: i32, mut b: i32) -> T {
        if node.is_terminal() {
            return node.clone();
        }
        let mut mm_node;
        let mut mm_score;
        let mut best_node = node.clone();
        let mut score = match node.is_maximising() {
            true => -100,
            false => 100,
        };
        for child_node in node.children() {
            let child_hash = calculate_hash(&child_node);
            if !self.scores.contains_key(&child_hash) {
                mm_node = self.alphabeta(&child_node, a, b);
                mm_score = mm_node.score();
                self.scores.insert(child_hash, mm_score);
                println!("Inserting: {} {}", child_hash, mm_score);
            } else {
                println!("Retrieving: {}", child_hash);
                mm_node = child_node;
                mm_score = mm_node.score();
            }
            if node.is_maximising() {
                if mm_score > score {
                    best_node = mm_node;
                    score = mm_score;
                }
                if score >= b {
                    break;
                }
                if score > a {
                    a = score;
                }
            } else {
                if mm_score < score {
                    score = mm_score;
                    best_node = mm_node;
                }
                if score <= a {
                    break;
                }
                if score < b {
                    b = score;
                }
            }
        }
        best_node
    }
}

//! Minimax search algorithm with alpha beta pruning.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait Node: Sized + Clone + Hash {
    fn depth(&self) -> usize;
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

    pub fn alphabeta<T: Node>(&mut self, node: &T, mut a: i32, mut b: i32) -> i32 {
        let node_hash = calculate_hash(&node);
        if self.scores.contains_key(&node_hash) {
            return *self.scores.get(&node_hash).unwrap();
        }
        if node.is_terminal() {
            let node_score = node.score();
            self.scores.insert(node_hash, node_score);
            return node_score;
        }
        let mut score = match node.is_maximising() {
            true => -100,
            false => 100,
        };
        for child_node in node.children() {
            let mm_score = self.alphabeta(&child_node, a, b);
            if node.is_maximising() {
                if mm_score > score {
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
                }
                if score <= a {
                    break;
                }
                if score < b {
                    b = score;
                }
            }
        }
        score
    }
}

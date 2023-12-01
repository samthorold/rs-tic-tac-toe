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
        if node.is_terminal() {
            return node.score();
        }
        // let mut mm_node;
        let mut mm_score;
        // let mut best_node = node.clone();
        let mut score = match node.is_maximising() {
            true => -100,
            false => 100,
        };
        for child_node in node.children() {
            let child_hash = calculate_hash(&child_node);
            if !self.scores.contains_key(&child_hash) {
                // mm_node = self.alphabeta(&child_node, a, b);
                // mm_score = mm_node.score();
                mm_score = self.alphabeta(&child_node, a, b)
                // self.scores.insert(child_hash, mm_score);
            } else {
                // mm_node = child_node.clone();
                mm_score = *self.scores.get(&child_hash).unwrap();
            }
            if node.is_maximising() {
                if mm_score > score {
                    // best_node = mm_node;
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
                    // best_node = mm_node;
                }
                if score <= a {
                    break;
                }
                if score < b {
                    b = score;
                }
            }
        }
        // best_node.score()
        score
    }
}

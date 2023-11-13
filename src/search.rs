use std::cmp::{max, min, Ordering};

use crate::game::{CellAddr, CellValue, GameState};

const NODE_MAX_SCORE: i32 = 100;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Node {
    pub state: GameState,
    pub moves: Vec<CellAddr>,
    pub is_maximum: bool,
    pub is_minimum: bool,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn children(&self, state: &GameState) -> Vec<(Node, GameState)> {
        let mut nodes = Vec::new();
        for child in state.children() {
            let mut moves = Vec::new();
            moves.clone_from(&self.moves);
            moves.push(child.clone().last_move.unwrap());
            nodes.push((
                Node {
                    state: child.clone(),
                    moves,
                    is_maximum: false,
                    is_minimum: false,
                },
                child.clone(),
            ));
        }
        nodes
    }
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }
    fn score(&self) -> i32 {
        if self.is_maximum {
            return NODE_MAX_SCORE;
        }
        if self.is_minimum {
            return -NODE_MAX_SCORE;
        }
        let score = self.state.score();
        if score < 0 {
            return score + self.state.depth() as i32;
        }
        if score > 0 {
            return score - self.state.depth() as i32;
        }
        score
    }
    fn is_maximising(&self) -> bool {
        return self.state.to_play == CellValue::O;
    }
    pub fn minimum(&self) -> Node {
        return Node {
            state: GameState::new(),
            moves: Vec::new(),
            is_maximum: false,
            is_minimum: true,
        };
    }
    pub fn maximum(&self) -> Node {
        return Node {
            state: GameState::new(),
            moves: Vec::new(),
            is_maximum: true,
            is_minimum: false,
        };
    }
}

pub fn minimax(state: &GameState, node: &Node, mut a: Node, mut b: Node) -> Node {
    if node.is_terminal() {
        return node.clone();
    }
    let mut best_node = match node.is_maximising() {
        true => node.minimum(),
        false => node.maximum(),
    };
    for (child_node, child_state) in node.children(state) {
        let minimax_value = minimax(&child_state, &child_node, a.clone(), b.clone());
        if node.is_maximising() {
            best_node = max(best_node.clone(), minimax_value.clone());
            a = max(a.clone(), best_node.clone());
            if best_node >= b.clone() {
                return best_node.clone();
            }
        } else {
            best_node = min(best_node.clone(), minimax_value);
            b = min(b.clone(), best_node.clone());
            if best_node <= a.clone() {
                return best_node.clone();
            }
        };
    }
    best_node
}

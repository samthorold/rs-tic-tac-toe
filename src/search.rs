use crate::game::{CellAddr, CellValue, GameState};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Node {
    pub state: GameState,
    pub moves: Vec<CellAddr>,
}

impl Node {
    fn children(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        for next_move in self.state.next_moves() {
            let next_state = self.state.next_state(next_move);
            let mut moves = Vec::new();
            moves.clone_from(&self.moves);
            moves.push(next_move.clone());
            nodes.push(Node {
                state: next_state,
                moves,
            });
        }
        nodes
    }
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }
    fn score(&self) -> i32 {
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
}

pub fn minimax(node: &Node) -> Node {
    if node.is_terminal() {
        return node.clone();
    }
    if node.is_maximising() {
        let mut score = -100;
        let mut best_node = node.clone();
        for child_node in node.children() {
            let mm_node = minimax(&child_node);
            let mm_score = mm_node.score();
            if mm_score > score {
                best_node = mm_node;
            }
            if mm_score > score {
                score = mm_score;
            }
        }
        return best_node;
    } else {
        let mut score = 100;
        let mut best_node = node.clone();
        for child_node in node.children() {
            let mm_node = minimax(&child_node);
            let mm_score = mm_node.score();
            if mm_score < score {
                best_node = mm_node;
            }
            if mm_score < score {
                score = mm_score;
            }
        }
        return best_node;
    }
}

pub fn alphabeta(node: &Node, mut a: i32, mut b: i32) -> Node {
    if node.is_terminal() {
        return node.clone();
    }
    if node.is_maximising() {
        let mut score = -100;
        let mut best_node = node.clone();
        for child_node in node.children() {
            let mm_node = alphabeta(&child_node, a, b);
            let mm_score = mm_node.score();
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
        }
        best_node
    } else {
        let mut score = 100;
        let mut best_node = node.clone();
        for child_node in node.children() {
            let mm_node = alphabeta(&child_node, a, b);
            let mm_score = mm_node.score();
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
        best_node
    }
}

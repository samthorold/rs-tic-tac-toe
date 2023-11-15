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
            // println!("{:?}", moves);
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

pub fn minimax(node: &Node, mut a: i32, mut b: i32) -> Node {
    // println!(
    //     "Entering minimax depth: {}, score: {}, a: {}, b: {}\n{}",
    //     node.state.depth(),
    //     node.score(),
    //     a,
    //     b,
    //     node.state
    // );
    if node.is_terminal() {
        // println!("Terminal");
        return node.clone();
    }
    let children = node.children();
    if children.len() == 0 {
        panic!("Not terminal and no children.")
    }
    let mut best_node = node.clone();
    for child_node in children {
        let mm_node = minimax(&child_node, a, b);
        println!(
            "Just left minimax depth: {}, score: {}, a: {}, b: {}\n{}",
            mm_node.state.depth(),
            mm_node.score(),
            a,
            b,
            mm_node.state
        );
        let mm_score = mm_node.score();
        if node.is_maximising() {
            if mm_score > best_node.score() {
                best_node = mm_node;
                // println!(
                //     "depth: {}, score: {}, a: {}, b: {}\n{}",
                //     best_node.state.depth(),
                //     best_node.score(),
                //     a,
                //     b,
                //     node.state
                // );
            }
            if mm_score > a {
                // println!("a: {}", mm_score);
                // println!(
                //     "depth: {}, score: {}, a: {}, b: {}\n{}",
                //     best_node.state.depth(),
                //     best_node.score(),
                //     a,
                //     b,
                //     node.state
                // );
                a = mm_score;
            }
            if best_node.score() >= b {
                // println!("node >= b {}\n{}", b, best_node.state);
                // println!(
                //     "depth: {}, score: {}, a: {}, b: {}\n{}",
                //     best_node.state.depth(),
                //     best_node.score(),
                //     a,
                //     b,
                //     node.state
                // );
                break;
            }
        } else {
            if mm_score < best_node.score() {
                best_node = mm_node;
                // println!(
                //     "depth: {}, score: {}, a: {}, b: {}\n{}",
                //     best_node.state.depth(),
                //     best_node.score(),
                //     a,
                //     b,
                //     node.state
                // );
            }
            if mm_score < b {
                // println!("b: {}", mm_score);
                // println!(
                //     "depth: {}, score: {}, a: {}, b: {}\n{}",
                //     best_node.state.depth(),
                //     best_node.score(),
                //     a,
                //     b,
                //     node.state
                // );
                b = mm_score;
            }
            if best_node.score() <= a {
                // println!("node <= b {}\n{}", a, best_node.state);
                // println!(
                //     "Entering minimax depth: {}, score: {}, a: {}, b: {}\n{}",
                //     best_node.state.depth(),
                //     best_node.score(),
                //     a,
                //     b,
                //     node.state
                // );
                break;
            }
        };
    }
    // println!("Exiting minimax\n{}", best_node.state);
    println!(
        "Exiting minimax depth: {}, score: {}, a: {}, b: {}\n{}",
        best_node.state.depth(),
        best_node.score(),
        a,
        b,
        best_node.state
    );
    best_node
}

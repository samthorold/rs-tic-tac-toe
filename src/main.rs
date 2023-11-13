use crate::game::GameState;
use crate::search::minimax;
use crate::search::Node;

mod game;
mod search;

fn main() {
    let mut game = GameState::new();
    while !game.is_terminal() {
        println!("{}", game.depth());
        let node = Node {
            state: game.clone(),
            moves: Vec::new(),
            is_maximum: false,
            is_minimum: false,
        };
        let variation = minimax(&game, &node, node.minimum(), node.maximum());
        println!("{:?}", variation.moves);
        let next_move = variation.moves[game.depth()].clone();
        println!("{:?}\n", next_move);
        game = game.next_state(next_move.row, next_move.col);
        println!("{}", game);
    }
}

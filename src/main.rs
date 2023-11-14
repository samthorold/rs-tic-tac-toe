use crate::game::GameState;
use crate::search::minimax;
use crate::search::Node;

mod game;
mod search;

fn main() {
    let mut game = GameState::new();
    println!("{}", game);
    while !game.is_terminal() {
        println!("{}", game.depth());
        let node = Node {
            state: game.clone(),
            moves: Vec::new(),
        };
        let variation = minimax(&node, -100, 100);
        println!("{:?}", variation.moves);
        let next_move = variation.moves[game.depth()].clone();
        println!("{:?}\n", next_move);
        game = game.next_state(&next_move);
        println!("{}", game);
    }
}

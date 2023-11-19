use crate::game::{CellAddr, GameState};
use crate::search::Node;
use crate::search::{alphabeta, minimax};

mod game;
mod search;

fn main() {
    let mut game = GameState::new()
        .next_state(&CellAddr { row: 1, col: 1 })
        .next_state(&CellAddr { row: 2, col: 2 });
    // .next_state(&CellAddr { row: 3, col: 1 });
    println!("{}", game);
    while !game.is_terminal() {
        let node = Node {
            state: game.clone(),
            moves: Vec::new(),
        };
        let variation = alphabeta(&node, -100, 100);
        // let variation = minimax(&node);
        println!("{:?}", variation.moves);
        let next_move = variation.moves[0].clone();
        println!("{:?}\n", next_move);
        game = game.next_state(&next_move);
        println!("{}", game);
    }
}

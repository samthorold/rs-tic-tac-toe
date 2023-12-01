use std::io;

use crate::game::{GameNode, GameState, Position, Value};
use crate::search::Search;

pub trait Player {
    fn next_move(&mut self, game: &GameState) -> Position;
}

pub struct AutoPlayer {
    maximising_player: Value,
    search: Search,
}

impl Player for AutoPlayer {
    fn next_move(&mut self, game: &GameState) -> Position {
        let node = GameNode {
            state: game.clone(),
            maximising_player: self.maximising_player,
            moves: Vec::new(),
        };
        let variation = self.search.alphabeta(&node, -100, 100);
        // implements Copy
        variation.moves[0]
    }
}

impl AutoPlayer {
    pub fn new(maximising_player: Value) -> AutoPlayer {
        AutoPlayer {
            maximising_player,
            search: Search::new(),
        }
    }
}

pub struct InteractivePlayer {}

impl Player for InteractivePlayer {
    fn next_move(&mut self, _game: &GameState) -> Position {
        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("Expected a move.");
        let mut itr = next_move.chars();
        let row = usize::try_from(itr.next().unwrap().to_digit(10).unwrap()).unwrap();
        let col = usize::try_from(itr.next().unwrap().to_digit(10).unwrap()).unwrap();
        Position { row, col }
    }
}

use std::io;

use crate::game::{CellAddr, CellValue, GameNode, GameState};
use crate::search::Search;

pub trait Player {
    fn next_move(&mut self, game: &GameState) -> CellAddr;
}

pub struct AutoPlayer {
    maximising_player: CellValue,
    search: Search,
}

impl Player for AutoPlayer {
    fn next_move(&mut self, game: &GameState) -> CellAddr {
        let mut actual_game = game.clone();
        actual_game.maximising_player = self.maximising_player;
        let node = GameNode {
            state: actual_game,
            moves: Vec::new(),
        };
        let variation = self.search.alphabeta(&node, -100, 100);
        variation.moves[0].clone()
    }
}

impl AutoPlayer {
    pub fn new(maximising_player: CellValue) -> AutoPlayer {
        AutoPlayer {
            maximising_player,
            search: Search::new(),
        }
    }
}

pub struct InteractivePlayer {}

impl Player for InteractivePlayer {
    fn next_move(&mut self, _game: &GameState) -> CellAddr {
        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("Expected a move.");
        let mut itr = next_move.chars();
        let row = usize::try_from(itr.next().unwrap().to_digit(10).unwrap()).unwrap();
        let col = usize::try_from(itr.next().unwrap().to_digit(10).unwrap()).unwrap();
        CellAddr { row, col }
    }
}

use std::io;

use crate::{
    game::{CellAddr, GameState},
    search::{alphabeta, Node},
};

pub struct AutoPlayer {}

impl AutoPlayer {
    pub fn next_move(&self, game: &GameState) -> CellAddr {
        let node = Node {
            state: game.clone(),
            moves: Vec::new(),
        };
        let variation = alphabeta(&node, -100, 100);
        variation.moves[0].clone()
    }
}

pub struct InteractivePlayer {}

impl InteractivePlayer {
    pub fn next_move(&self, _game: &GameState) -> CellAddr {
        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("Expected a move.");
        let mut itr = next_move.chars();
        let row = usize::try_from(itr.next().unwrap().to_digit(10).unwrap()).unwrap();
        let col = usize::try_from(itr.next().unwrap().to_digit(10).unwrap()).unwrap();
        println!("{} {}", row, col);
        CellAddr { row, col }
    }
}

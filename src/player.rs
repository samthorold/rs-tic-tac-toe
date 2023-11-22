use std::io;

use crate::{
    game::{CellAddr, GameNode, GameState},
    search::alphabeta,
};

pub trait Player {
    fn next_move(&self, game: &GameState) -> CellAddr;
}

pub struct AutoPlayer {}

impl Player for AutoPlayer {
    fn next_move(&self, game: &GameState) -> CellAddr {
        let node = GameNode {
            state: game.clone(),
            moves: Vec::new(),
        };
        let variation = alphabeta(&node, -100, 100);
        variation.moves[0].clone()
    }
}

pub struct InteractivePlayer {}

impl Player for InteractivePlayer {
    fn next_move(&self, _game: &GameState) -> CellAddr {
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

//! Implementations for different tic tac toe players.

use core::panic;
use std::io;

use crate::game::{GameNode, GameState, Position};
use crate::search::{Node, Search};

pub trait Player {
    fn next_move(&mut self, game: &GameState) -> Position;
}

pub struct AutoPlayer {
    search: Search,
}

impl Player for AutoPlayer {
    fn next_move(&mut self, game: &GameState) -> Position {
        let node = GameNode {
            state: game.clone(),
            moves: Vec::new(),
        };
        let mut best_child: Option<GameNode> = None;
        let mut best_score = match node.is_maximising() {
            true => -100,
            false => 100,
        };
        for child in node.children() {
            let score = self.search.alphabeta(&child, -100, 100);
            if node.is_maximising() {
                if score > best_score {
                    best_child = Some(child);
                    best_score = score;
                }
            } else {
                if score < best_score {
                    best_child = Some(child);
                    best_score = score;
                }
            }
        }
        match best_child {
            Some(child) => child.moves[0],
            _ => panic!("Expected a best child node"),
        }
        // best_child.moves[0]
        // let variation = self.search.alphabeta(&node, -100, 100);
        // variation.moves[0]
    }
}

impl AutoPlayer {
    pub fn new() -> AutoPlayer {
        AutoPlayer {
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

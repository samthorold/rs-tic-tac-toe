use std::io;

use crate::game::GameState;
use crate::player::{AutoPlayer, InteractivePlayer, Player};

mod game;
mod player;
mod search;

fn main() {
    let mut plyrs = String::new();
    io::stdin()
        .read_line(&mut plyrs)
        .expect("Failed to read line");
    let mut itr = plyrs.chars();
    let player1: Box<dyn Player> = match itr.next().unwrap() {
        'i' => Box::new(InteractivePlayer {}),
        'a' => Box::new(AutoPlayer::new()),
        _ => panic!("Unknown player type"),
    };
    let player2: Box<dyn Player> = match itr.next().unwrap() {
        'i' => Box::new(InteractivePlayer {}),
        'a' => Box::new(AutoPlayer::new()),
        _ => panic!("Unknown player type"),
    };

    let mut game = GameState::new();
    println!("{}", game);
    let mut players: Vec<Box<dyn Player>> = Vec::new();
    players.push(player1);
    players.push(player2);
    while !game.is_terminal() {
        let is_player1 = game.depth % 2 == 0;
        if is_player1 {
            let next_move = players[0].next_move(&game);
            game = game.next_state(&next_move);
        } else {
            let next_move = players[1].next_move(&game);
            game = game.next_state(&next_move);
        }
        println!("{}", game);
    }
}

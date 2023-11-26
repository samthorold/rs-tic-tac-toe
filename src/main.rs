use std::io;

use game::CellAddr;

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
        'a' => Box::new(AutoPlayer::new(game::CellValue::O)),
        _ => panic!("Unknown player type"),
    };
    let player2: Box<dyn Player> = match itr.next().unwrap() {
        'i' => Box::new(InteractivePlayer {}),
        'a' => Box::new(AutoPlayer::new(game::CellValue::X)),
        _ => panic!("Unknown player type"),
    };

    let mut game = GameState::new(game::CellValue::O);
    // .next_state(&CellAddr { row: 1, col: 1 })
    // .next_state(&CellAddr { row: 1, col: 2 });
    // .next_state(&CellAddr { row: 1, col: 2 });
    println!("{}", game);
    let mut players: Vec<Box<dyn Player>> = Vec::new();
    players.push(player1);
    players.push(player2);
    let mut is_player1 = true;
    while !game.is_terminal() {
        if is_player1 {
            game = game.next_state(&players[0].next_move(&game));
        } else {
            game = game.next_state(&players[1].next_move(&game));
        }
        println!("{}", game);
        is_player1 = !is_player1;
    }
}

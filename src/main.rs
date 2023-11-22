use crate::game::GameState;
use crate::player::{AutoPlayer, InteractivePlayer, Player};

mod game;
mod player;
mod search;

fn main() {
    let mut game = GameState::new();
    let player1 = InteractivePlayer {};
    let player2 = AutoPlayer {};
    let mut players: Vec<Box<dyn Player>> = Vec::new();
    players.push(Box::new(player1));
    players.push(Box::new(player2));
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

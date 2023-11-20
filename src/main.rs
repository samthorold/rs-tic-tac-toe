use crate::game::GameState;
use crate::player::{AutoPlayer, InteractivePlayer};

mod game;
mod player;
mod search;

fn main() {
    let mut game = GameState::new();
    let player1 = InteractivePlayer {};
    let player2 = AutoPlayer {};
    let mut is_player1 = true;
    while !game.is_terminal() {
        if is_player1 {
            game = game.next_state(&player1.next_move(&game));
        } else {
            game = game.next_state(&player2.next_move(&game));
        }
        println!("{}", game);
        is_player1 = !is_player1;
    }
}

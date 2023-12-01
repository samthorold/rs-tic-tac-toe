use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::search::Node;

const MAX_SCORE: i32 = 10;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Value {
    X,
    O,
    N,
}

const PLAYERS: [Value; 2] = [Value::O, Value::X];

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Cell {
    addr: Position,
    value: Value,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct GameState {
    cells: [[Cell; 3]; 3],
    pub depth: usize,
    pub to_play: Value,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in &self.cells {
            for cell in row {
                match cell.value {
                    Value::X => s.push('x'),
                    Value::O => s.push('o'),
                    Value::N => s.push('.'),
                };
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl GameState {
    pub fn new() -> GameState {
        let mut cells = [[Cell {
            addr: Position { row: 1, col: 1 },
            value: Value::N,
        }; 3]; 3];
        for row in 1..4 {
            for col in 1..4 {
                cells[row - 1][col - 1].addr.row = row;
                cells[row - 1][col - 1].addr.col = col;
            }
        }
        GameState {
            cells,
            depth: 0,
            to_play: Value::O,
        }
    }
    fn next_player(&self) -> Value {
        match self.to_play {
            Value::X => Value::O,
            Value::O => Value::X,
            Value::N => panic!("Should not have N to play."),
        }
    }
    pub fn next_state(&self, addr: &Position) -> GameState {
        let row = addr.row;
        let col = addr.col;
        if !(1..=9).contains(&row) {
            panic!("Row value invalid.")
        }
        if !(1..=9).contains(&col) {
            panic!("Row value invalid.")
        }
        let mut cells = self.cells;
        cells[row - 1][col - 1] = Cell {
            addr: Position { row, col },
            value: self.to_play,
        };
        GameState {
            cells,
            depth: self.depth + 1,
            to_play: self.next_player(),
        }
    }
    pub fn next_moves(&self) -> Vec<&Position> {
        let mut addrs = Vec::new();
        for row in &self.cells {
            for cell in row {
                if cell.value == Value::N {
                    addrs.push(&cell.addr);
                }
            }
        }
        addrs
    }
    pub fn score(&self) -> i32 {
        for player in PLAYERS {
            let sign = match player == Value::O {
                true => 1,
                false => -1,
            };
            for row in 1..4 {
                let all = self
                    .cells
                    .iter()
                    .flatten()
                    .filter(|cell| cell.addr.row == row)
                    .all(|cell| cell.value == player);
                if all {
                    return sign * MAX_SCORE;
                }
            }
            for col in 1..4 {
                let all = self
                    .cells
                    .iter()
                    .flatten()
                    .filter(|cell| cell.addr.col == col)
                    .all(|cell| cell.value == player);
                if all {
                    return sign * MAX_SCORE;
                }
            }
            let all = self
                .cells
                .iter()
                .flatten()
                .filter(|cell| {
                    (cell.addr.row == 1 && cell.addr.col == 1)
                        | (cell.addr.row == 2 && cell.addr.col == 2)
                        | (cell.addr.row == 3 && cell.addr.col == 3)
                })
                .all(|cell| cell.value == player);
            if all {
                return sign * MAX_SCORE;
            }
            let all = self
                .cells
                .iter()
                .flatten()
                .filter(|cell| {
                    (cell.addr.row == 3 && cell.addr.col == 1)
                        | (cell.addr.row == 2 && cell.addr.col == 2)
                        | (cell.addr.row == 1 && cell.addr.col == 3)
                })
                .all(|cell| cell.value == player);
            if all {
                return sign * MAX_SCORE;
            }
        }
        0
    }
    pub fn is_terminal(&self) -> bool {
        (self.depth == 9) | (self.score() != 0)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GameNode {
    pub state: GameState,
    pub moves: Vec<Position>,
}

impl Hash for GameNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.cells.hash(state);
    }
}

impl Display for GameNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl Node for GameNode {
    fn depth(&self) -> usize {
        self.state.depth
    }
    fn children(&self) -> Vec<GameNode> {
        let mut nodes = Vec::new();
        for next_move in self.state.next_moves() {
            let next_state = self.state.next_state(next_move);
            let mut moves = Vec::new();
            moves.clone_from(&self.moves);
            moves.push(*next_move);
            nodes.push(GameNode {
                state: next_state,
                moves,
            });
        }
        nodes
    }
    fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }
    fn score(&self) -> i32 {
        let score = self.state.score();
        if score < 0 {
            return score + self.state.depth as i32;
        }
        if score > 0 {
            return score - self.state.depth as i32;
        }
        score
    }
    fn is_maximising(&self) -> bool {
        self.state.to_play == Value::O
    }
}

#[cfg(test)]
mod test_score {
    use super::*;
    #[test]
    fn new_game() {
        let game = GameState::new();
        assert_eq!(game.score(), 0);
        assert_eq!(game.is_terminal(), false);
        assert_eq!(game.to_play, Value::O);
    }
    #[test]
    fn one_move() {
        let game = GameState::new().next_state(&Position { row: 1, col: 1 });
        assert_eq!(game.score(), 0);
        assert_eq!(game.is_terminal(), false);
    }
    #[test]
    fn x_wins_row() {
        let game = GameState::new()
            .next_state(&Position { row: 1, col: 1 })
            .next_state(&Position { row: 2, col: 1 })
            .next_state(&Position { row: 1, col: 2 })
            .next_state(&Position { row: 2, col: 2 })
            .next_state(&Position { row: 1, col: 3 });
        assert_eq!(game.score(), 10);
        assert_eq!(game.is_terminal(), true);
    }
    #[test]
    fn x_wins_diag_from_top_left() {
        let game = GameState::new()
            .next_state(&Position { row: 1, col: 1 })
            .next_state(&Position { row: 1, col: 2 })
            .next_state(&Position { row: 2, col: 2 })
            .next_state(&Position { row: 2, col: 3 })
            .next_state(&Position { row: 3, col: 3 });
        assert_eq!(game.score(), 10);
        assert_eq!(game.is_terminal(), true);
    }
    #[test]
    fn x_wins_diag_from_bottom_left() {
        let game = GameState::new()
            .next_state(&Position { row: 3, col: 1 })
            .next_state(&Position { row: 1, col: 2 })
            .next_state(&Position { row: 2, col: 2 })
            .next_state(&Position { row: 2, col: 3 })
            .next_state(&Position { row: 1, col: 3 });
        assert_eq!(game.score(), 10);
        assert_eq!(game.is_terminal(), true);
    }
}

#[cfg(test)]
mod test_is_maximising {
    use super::*;
    #[test]
    fn test_o_is_maximising() {
        let game = GameState::new();
        let node = GameNode {
            state: game,
            moves: Vec::new(),
        };
        assert!(node.is_maximising());
    }
    #[test]
    fn test_x_is_maximising_move_2() {
        let game = GameState::new().next_state(&Position { row: 1, col: 1 });
        let node = GameNode {
            state: game,
            moves: Vec::new(),
        };
        assert!(!node.is_maximising());
    }
}

#[cfg(test)]
mod test_next_moves {
    use super::*;
    #[test]
    fn test_next_moves() {
        let game = GameState::new().next_state(&Position { row: 1, col: 1 });
        let next_moves = game.next_moves();
        let next_move = next_moves.iter().take(1).last().unwrap();
        assert_eq!(**next_move, Position { row: 1, col: 2 });
    }
}

#[cfg(test)]
mod test_hash {
    use std::collections::hash_map::DefaultHasher;

    use super::*;

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    #[test]
    fn test_game_state_hash() {
        let game1 = GameState::new().next_state(&Position { row: 1, col: 1 });
        let game2 = GameState::new().next_state(&Position { row: 1, col: 1 });
        assert_eq!(calculate_hash(&game1), calculate_hash(&game2));
    }
}

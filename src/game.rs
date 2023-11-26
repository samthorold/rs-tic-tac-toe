use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::search::Node;

const MAX_SCORE: i32 = 10;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct CellAddr {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum CellValue {
    X,
    O,
    N,
}

const PLAYERS: [CellValue; 2] = [CellValue::O, CellValue::X];

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Cell {
    addr: CellAddr,
    value: CellValue,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct GameState {
    cells: [[Cell; 3]; 3],
    pub to_play: CellValue,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in &self.cells {
            for cell in row {
                match cell.value {
                    CellValue::X => s.push('x'),
                    CellValue::O => s.push('o'),
                    CellValue::N => s.push('.'),
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
            addr: CellAddr { row: 1, col: 1 },
            value: CellValue::N,
        }; 3]; 3];
        for row in 1..4 {
            for col in 1..4 {
                cells[row - 1][col - 1].addr.row = row;
                cells[row - 1][col - 1].addr.col = col;
            }
        }
        GameState {
            cells,
            to_play: CellValue::O,
        }
    }
    fn next_player(&self) -> CellValue {
        match self.to_play {
            CellValue::X => CellValue::O,
            CellValue::O => CellValue::X,
            CellValue::N => panic!("Should not have N to play."),
        }
    }
    pub fn next_state(&self, addr: &CellAddr) -> GameState {
        let row = addr.row;
        let col = addr.col;
        if (row < 1) | (row > 9) {
            panic!("Row value invalid.")
        }
        if (col < 1) | (col > 9) {
            panic!("Row value invalid.")
        }
        let mut cells = self.cells.clone();
        cells[row - 1][col - 1] = Cell {
            addr: CellAddr { row, col },
            value: self.to_play,
        };
        GameState {
            cells,
            to_play: self.next_player(),
        }
    }
    pub fn next_moves(&self) -> Vec<&CellAddr> {
        let mut addrs = Vec::new();
        for row in &self.cells {
            for cell in row {
                if cell.value == CellValue::N {
                    addrs.push(&cell.addr);
                }
            }
        }
        addrs
    }
    pub fn depth(&self) -> usize {
        let mut count = 0;
        for row in self.cells {
            for cell in row {
                if cell.value != CellValue::N {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn score(&self) -> i32 {
        for player in PLAYERS {
            let sign = match player {
                CellValue::O => 1,
                CellValue::X => -1,
                _ => panic!("Not a player."),
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
        (self.depth() == 9) | (self.score() != 0)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GameNode {
    pub state: GameState,
    pub moves: Vec<CellAddr>,
}

impl Hash for GameNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
        // self.moves.hash(state);
    }
}

impl Node for GameNode {
    fn children(&self) -> Vec<GameNode> {
        let mut nodes = Vec::new();
        for next_move in self.state.next_moves() {
            let next_state = self.state.next_state(next_move);
            let mut moves = Vec::new();
            moves.clone_from(&self.moves);
            moves.push(next_move.clone());
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
            return score + self.state.depth() as i32;
        }
        if score > 0 {
            return score - self.state.depth() as i32;
        }
        score
    }
    fn is_maximising(&self) -> bool {
        return self.state.to_play == CellValue::O;
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
        assert_eq!(game.to_play, CellValue::O);
    }
    #[test]
    fn one_move() {
        let game = GameState::new().next_state(&CellAddr { row: 1, col: 1 });
        assert_eq!(game.score(), 0);
        assert_eq!(game.is_terminal(), false);
    }
    #[test]
    fn x_wins_row() {
        let game = GameState::new()
            .next_state(&CellAddr { row: 1, col: 1 })
            .next_state(&CellAddr { row: 2, col: 1 })
            .next_state(&CellAddr { row: 1, col: 2 })
            .next_state(&CellAddr { row: 2, col: 2 })
            .next_state(&CellAddr { row: 1, col: 3 });
        assert_eq!(game.score(), 10);
        assert_eq!(game.is_terminal(), true);
    }
    #[test]
    fn x_wins_col() {
        let game = GameState::new()
            .next_state(&CellAddr { row: 1, col: 1 })
            .next_state(&CellAddr { row: 1, col: 2 })
            .next_state(&CellAddr { row: 2, col: 1 })
            .next_state(&CellAddr { row: 2, col: 2 })
            .next_state(&CellAddr { row: 3, col: 1 });
        assert_eq!(game.score(), 10);
        assert_eq!(game.is_terminal(), true);
    }
    #[test]
    fn x_wins_diag_from_top_left() {
        let game = GameState::new()
            .next_state(&CellAddr { row: 1, col: 1 })
            .next_state(&CellAddr { row: 1, col: 2 })
            .next_state(&CellAddr { row: 2, col: 2 })
            .next_state(&CellAddr { row: 2, col: 3 })
            .next_state(&CellAddr { row: 3, col: 3 });
        assert_eq!(game.score(), 10);
        assert_eq!(game.is_terminal(), true);
    }
    #[test]
    fn x_wins_diag_from_bottom_left() {
        let game = GameState::new()
            .next_state(&CellAddr { row: 3, col: 1 })
            .next_state(&CellAddr { row: 1, col: 2 })
            .next_state(&CellAddr { row: 2, col: 2 })
            .next_state(&CellAddr { row: 2, col: 3 })
            .next_state(&CellAddr { row: 1, col: 3 });
        assert_eq!(game.score(), 10);
        assert_eq!(game.is_terminal(), true);
    }
}

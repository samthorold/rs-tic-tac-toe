use std::{
    cmp::{max, min, Ordering},
    env,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

const MAX_SCORE: usize = 10;
const NODE_MAX_SCORE: i32 = 100;
const NODE_MIN_SCORE: i32 = -100;

#[derive(Debug)]
enum PlayerKind {
    Interactive,
    Computer,
}

impl FromStr for PlayerKind {
    type Err = ();
    fn from_str(s: &str) -> Result<PlayerKind, Self::Err> {
        match s {
            "interactive" => Ok(PlayerKind::Interactive),
            "computer" => Ok(PlayerKind::Computer),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct CliArgs {
    player1: PlayerKind,
    player2: PlayerKind,
}

impl CliArgs {
    fn from_args(args: env::Args) -> CliArgs {
        let args: Vec<String> = args.collect();
        CliArgs {
            player1: PlayerKind::from_str(&args[1]).unwrap(),
            player2: PlayerKind::from_str(&args[2]).unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct CellAddr {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum CellValue {
    X,
    O,
    N,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Cell {
    addr: CellAddr,
    value: CellValue,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct GameState {
    cells: [Cell; 9],
    to_play: CellValue,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for cell in &self.cells {
            match cell.value {
                CellValue::X => s.push('x'),
                CellValue::O => s.push('o'),
                CellValue::N => s.push('.'),
            };
            if cell.addr.col == 3 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

impl GameState {
    fn new() -> GameState {
        let mut cells = Vec::new();
        for row in 1..4 {
            for col in 1..4 {
                cells.push(Cell {
                    addr: CellAddr { row, col },
                    value: CellValue::N,
                })
            }
        }
        GameState {
            cells: <[Cell; 9]>::try_from(cells).unwrap(),
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
    fn next_state(&self, row: usize, col: usize) -> GameState {
        if (row < 1) | (row > 9) {
            panic!("Row value invalid.")
        }
        if (col < 1) | (col > 9) {
            panic!("Row value invalid.")
        }
        let mut cells = Vec::new();
        for cell in &self.cells {
            let is_changed_cell = (cell.addr.row == row) & (cell.addr.col == col);
            if is_changed_cell & (cell.value != CellValue::N) {
                panic!("Trying to set an already set cell.")
            }
            cells.push(Cell {
                addr: CellAddr {
                    row: cell.addr.row,
                    col: cell.addr.col,
                },
                value: match is_changed_cell {
                    true => self.to_play,
                    false => cell.value,
                },
            })
        }
        GameState {
            cells: <[Cell; 9]>::try_from(cells).unwrap(),
            to_play: self.next_player(),
        }
    }
    fn children(&self) -> Vec<GameState> {
        let mut cells = Vec::new();
        for cell in &self.cells {
            if cell.value == CellValue::N {
                cells.push(self.next_state(cell.addr.row, cell.addr.col));
            }
        }
        cells
    }
    fn score(&self) -> i32 {
        // The depth. "Punish" scores for a high depth.
        let count_n = self
            .cells
            .iter()
            .filter(|cell| cell.value != CellValue::N)
            .count();
        for row in 1..4 {
            let all_x = self
                .cells
                .iter()
                .filter(|cell| cell.addr.row == row)
                .all(|cell| cell.value == CellValue::X);
            if all_x {
                return -<i32>::try_from(MAX_SCORE - count_n).unwrap();
            }
            let all_o = self
                .cells
                .iter()
                .filter(|cell| cell.addr.row == row)
                .all(|cell| cell.value == CellValue::O);
            if all_o {
                return <i32>::try_from(MAX_SCORE - count_n).unwrap();
            }
        }
        for col in 1..4 {
            let all_x = self
                .cells
                .iter()
                .filter(|cell| cell.addr.col == col)
                .all(|cell| cell.value == CellValue::X);
            if all_x {
                return -<i32>::try_from(MAX_SCORE - count_n).unwrap();
            }
            let all_o = self
                .cells
                .iter()
                .filter(|cell| cell.addr.col == col)
                .all(|cell| cell.value == CellValue::O);
            if all_o {
                return <i32>::try_from(MAX_SCORE - count_n).unwrap();
            }
        }
        let all_x = self
            .cells
            .iter()
            .filter(|cell| {
                (cell.addr.row == 1 && cell.addr.col == 1)
                    | (cell.addr.row == 2 && cell.addr.col == 2)
                    | (cell.addr.row == 3 && cell.addr.col == 3)
            })
            .all(|cell| cell.value == CellValue::X);
        if all_x {
            return -<i32>::try_from(MAX_SCORE - count_n).unwrap();
        }
        let all_o = self
            .cells
            .iter()
            .filter(|cell| {
                (cell.addr.row == 1 && cell.addr.col == 1)
                    | (cell.addr.row == 2 && cell.addr.col == 2)
                    | (cell.addr.row == 3 && cell.addr.col == 3)
            })
            .all(|cell| cell.value == CellValue::O);
        if all_o {
            return <i32>::try_from(MAX_SCORE - count_n).unwrap();
        }
        let all_x = self
            .cells
            .iter()
            .filter(|cell| {
                (cell.addr.row == 3 && cell.addr.col == 1)
                    | (cell.addr.row == 2 && cell.addr.col == 2)
                    | (cell.addr.row == 1 && cell.addr.col == 3)
            })
            .all(|cell| cell.value == CellValue::X);
        if all_x {
            return -<i32>::try_from(MAX_SCORE - count_n).unwrap();
        }
        let all_o = self
            .cells
            .iter()
            .filter(|cell| {
                (cell.addr.row == 3 && cell.addr.col == 1)
                    | (cell.addr.row == 2 && cell.addr.col == 2)
                    | (cell.addr.row == 1 && cell.addr.col == 3)
            })
            .all(|cell| cell.value == CellValue::O);
        if all_o {
            return <i32>::try_from(MAX_SCORE - count_n).unwrap();
        }
        0
    }
    fn is_terminal(&self) -> bool {
        let free_cells = self
            .cells
            .iter()
            .filter(|cell| cell.value == CellValue::N)
            .count();
        (free_cells == 0) | (self.score() != 0)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Node {
    states: Vec<GameState>,
    is_maximum: bool,
    is_minimum: bool,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:#?} {} {} {} {}\n{}",
            self.latest_state().to_play,
            self.is_maximising(),
            self.score(),
            self.is_terminal(),
            self.states.len(),
            self.latest_state(),
        )
    }
}

impl Node {
    fn latest_state(&self) -> &GameState {
        self.states.last().expect("Expect some states.")
    }
    fn children(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        for state in self.latest_state().children() {
            let mut states = Vec::new();
            states.clone_from(&self.states);
            states.push(state);
            nodes.push(Node {
                states,
                is_maximum: false,
                is_minimum: false,
            });
        }
        nodes
    }
    fn to_play(&self) -> CellValue {
        self.latest_state().to_play
    }
    fn is_terminal(&self) -> bool {
        self.latest_state().is_terminal()
    }
    fn score(&self) -> i32 {
        if self.is_maximum {
            return NODE_MAX_SCORE;
        }
        if self.is_minimum {
            return NODE_MIN_SCORE;
        }
        return self.latest_state().score();
    }
    fn is_maximising(&self) -> bool {
        return self.to_play() == CellValue::O;
    }
    fn minimum(&self) -> Node {
        return Node {
            states: vec![GameState::new()],
            is_maximum: false,
            is_minimum: true,
        };
    }
    fn maximum(&self) -> Node {
        return Node {
            states: vec![GameState::new()],
            is_maximum: true,
            is_minimum: false,
        };
    }
}

fn minimax(node: &Node, mut a: Node, mut b: Node) -> Node {
    // println!("Examining\n{}", node);
    if node.is_terminal() {
        // println!("  Is terminal");
        return node.clone();
    }
    let mut best_node = match node.is_maximising() {
        true => node.minimum(),
        false => node.maximum(),
    };
    for child in node.children() {
        let minimax_value = minimax(&child, a.clone(), b.clone());
        // println!("{:#?} Minimax value\n{}", node.to_play(), minimax_value);
        // println!("Comparing to\n{}a\n{}b\n{}", best_node, a, b);
        if node.is_maximising() {
            best_node = max(best_node.clone(), minimax_value.clone());
            // println!("Best Node\n{}", best_node);
            a = max(a.clone(), best_node.clone());
            // println!("New a\n{}", a);
            if best_node >= b.clone() {
                // println!("Greater than b.");
                // println!("Returning {}", best_node);
                return best_node.clone();
            }
        } else {
            best_node = min(best_node.clone(), minimax_value);
            // println!("Best Node\n{}", best_node);
            b = min(b.clone(), best_node.clone());
            // println!("New b\n{}", b);
            if best_node <= a.clone() {
                // println!("Less than a.");
                // println!("Returning {}", best_node);
                return best_node.clone();
            }
        };
    }
    // println!("Returning {}", best_node);
    best_node
}

fn main() {
    let args = CliArgs::from_args(env::args());
    dbg!(&args);
    let game = GameState::new();
    let node = Node {
        states: vec![game],
        is_maximum: false,
        is_minimum: false,
    };
    // println!("{}", node);
    // for child in node.children() {
    //     println!("{}", child);
    //     if child.score() > 0 {
    //         println!("Latest state\n{}", child.latest_state());
    //         println!("-- Example states");
    //         for state in child.states {
    //             println!("{}", state);
    //         }
    //         println!("--");
    //     }
    // }

    let variation = minimax(&node, node.minimum(), node.maximum());
    println!("Node {}", variation);
    for state in variation.states {
        println!("{}", state);
    }
}

use std::{
    collections::HashMap,
    env,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

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
        }
    }
    fn next_value(&self) -> CellValue {
        let mut count: u8 = 9;
        for cell in &self.cells {
            if cell.value == CellValue::N {
                count -= 1;
            }
        }
        if count % 2 == 0 {
            return CellValue::O;
        } else {
            return CellValue::X;
        }
    }
    fn next_state(&self, row: usize, col: usize) -> GameState {
        if (row < 1) | (row > 9) {
            panic!("Row value invalid.")
        }
        if (col < 1) | (col > 9) {
            panic!("Row value invalid.")
        }
        let value = self.next_value();
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
                    true => value,
                    false => cell.value,
                },
            })
        }
        GameState {
            cells: <[Cell; 9]>::try_from(cells).unwrap(),
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
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    state: GameState,
    depth: i32,
}

impl Node {
    fn children(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        for state in self.state.children() {
            nodes.push(Node {
                state,
                depth: self.depth + 1,
            });
        }
        nodes
    }
}

/// What is the point of this?
/// 1. Don't want to calculate states on the fly all the time.
/// 2. Rust gets _super_ upset about tree-like structures.
struct Tree {
    tree: HashMap<Node, Vec<Node>>,
}

impl Tree {
    fn new(node: &Node) -> Tree {
        let mut tree = Tree {
            tree: HashMap::new(),
        };
        tree.tree.insert(node.clone(), node.children());
        tree
    }
    fn children(&mut self, node: &Node) -> &Vec<Node> {
        if !self.tree.contains_key(node) {
            self.tree.insert(node.clone(), node.children());
        }
        let children = self.tree.get(&node);
        match children {
            Some(children) => return children,
            None => {
                panic!("Somehow no children.")
            }
        }
    }
}

fn main() {
    let args = CliArgs::from_args(env::args());
    dbg!(&args);
    let game = GameState::new();
    // let game = game.next_state(1, 1);
    // println!("{}", game);
    // let game = game.next_state(1, 2);
    // println!("{}", game);
    // let game = game.next_state(3, 2);
    // println!("{}", game);
    // for c in game.children() {
    //     println!("{}", c)
    // }
    let node = Node {
        state: game,
        depth: 0,
    };

    let mut tree = Tree::new(&node);
    let children = tree.children(&node);
    for child in children {
        println!("{}", child.state)
    }
}

pub trait Node: Sized {
    fn children(&self) -> Vec<Self>;
    fn is_terminal(&self) -> bool;
    fn score(&self) -> i32;
    fn is_maximising(&self) -> bool;
}

pub fn alphabeta<T: Node + Clone>(node: &T, mut a: i32, mut b: i32) -> T {
    if node.is_terminal() {
        return node.clone();
    }
    let mut best_node = node.clone();
    if node.is_maximising() {
        let mut score = -100;
        for child_node in node.children() {
            let mm_node = alphabeta(&child_node, a, b);
            let mm_score = mm_node.score();
            if mm_score > score {
                best_node = mm_node;
                score = mm_score;
            }
            if score >= b {
                break;
            }
            if score > a {
                a = score;
            }
        }
        best_node
    } else {
        let mut score = 100;
        for child_node in node.children() {
            let mm_node = alphabeta(&child_node, a, b);
            let mm_score = mm_node.score();
            if mm_score < score {
                score = mm_score;
                best_node = mm_node;
            }
            if score <= a {
                break;
            }
            if score < b {
                b = score;
            }
        }
        best_node
    }
}

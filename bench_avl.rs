// Benchmark: 1M sequential inserts + 1M sequential deletes on AVL tree.

use std::time::Instant;

struct Node {
    value: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct AvlTree {
    root: Option<Box<Node>>,
}

impl Node {
    fn leaf(value: i32) -> Box<Self> {
        Box::new(Node { value, height: 1, left: None, right: None })
    }
    fn update_height(&mut self) {
        self.height = 1 + height(&self.left).max(height(&self.right));
    }
    fn balance_factor(&self) -> i32 {
        height(&self.left) - height(&self.right)
    }
}

fn height(node: &Option<Box<Node>>) -> i32 {
    node.as_ref().map_or(0, |n| n.height)
}

fn rotate_right(mut root: Box<Node>) -> Box<Node> {
    let mut new_root = root.left.take().unwrap();
    root.left = new_root.right.take();
    root.update_height();
    new_root.right = Some(root);
    new_root.update_height();
    new_root
}

fn rotate_left(mut root: Box<Node>) -> Box<Node> {
    let mut new_root = root.right.take().unwrap();
    root.right = new_root.left.take();
    root.update_height();
    new_root.left = Some(root);
    new_root.update_height();
    new_root
}

fn rebalance(mut node: Box<Node>) -> Box<Node> {
    node.update_height();
    let bf = node.balance_factor();
    if bf > 1 {
        if node.left.as_ref().unwrap().balance_factor() < 0 {
            node.left = Some(rotate_left(node.left.take().unwrap()));
        }
        rotate_right(node)
    } else if bf < -1 {
        if node.right.as_ref().unwrap().balance_factor() > 0 {
            node.right = Some(rotate_right(node.right.take().unwrap()));
        }
        rotate_left(node)
    } else {
        node
    }
}

fn insert_node(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    match node {
        None => Some(Node::leaf(value)),
        Some(mut n) => {
            if value < n.value {
                n.left = insert_node(n.left.take(), value);
            } else if value > n.value {
                n.right = insert_node(n.right.take(), value);
            } else {
                return Some(n);
            }
            Some(rebalance(n))
        }
    }
}

fn take_min(mut node: Box<Node>) -> (Option<Box<Node>>, i32) {
    if node.left.is_some() {
        let (new_left, min_value) = take_min(node.left.take().unwrap());
        node.left = new_left;
        (Some(rebalance(node)), min_value)
    } else {
        (node.right.take(), node.value)
    }
}

fn delete_node(node: Option<Box<Node>>, value: i32) -> (Option<Box<Node>>, bool) {
    let Some(mut n) = node else { return (None, false); };
    if value < n.value {
        let (new_left, found) = delete_node(n.left.take(), value);
        n.left = new_left;
        (Some(rebalance(n)), found)
    } else if value > n.value {
        let (new_right, found) = delete_node(n.right.take(), value);
        n.right = new_right;
        (Some(rebalance(n)), found)
    } else {
        let replacement = match (n.left.take(), n.right.take()) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(l), Some(r)) => {
                let (new_right, succ) = take_min(r);
                n.value = succ;
                n.left = Some(l);
                n.right = new_right;
                Some(rebalance(n))
            }
        };
        (replacement, true)
    }
}

impl AvlTree {
    pub fn new() -> Self { AvlTree { root: None } }
    pub fn insert(&mut self, value: i32) {
        self.root = insert_node(self.root.take(), value);
    }
    pub fn delete(&mut self, value: i32) -> bool {
        let (new_root, found) = delete_node(self.root.take(), value);
        self.root = new_root;
        found
    }
    pub fn height(&self) -> i32 { height(&self.root) }
}

fn main() {
    const N: i32 = 1_000_000;
    println!("Rust N={N} sequential inserts + N sequential deletes");
    for trial in 1..=3 {
        let mut tree = AvlTree::new();

        let t0 = Instant::now();
        for i in 0..N { tree.insert(i); }
        let t1 = Instant::now();
        let h_inserts = tree.height();

        for i in 0..N { tree.delete(i); }
        let t2 = Instant::now();
        let h_deletes = tree.height();

        let ins = t1.duration_since(t0).as_secs_f64() * 1000.0;
        let del = t2.duration_since(t1).as_secs_f64() * 1000.0;
        println!("  trial {trial}  insert={ins:7.1} ms  delete={del:7.1} ms  height_after_inserts={h_inserts:2}  height_after_deletes={h_deletes}");
    }
}

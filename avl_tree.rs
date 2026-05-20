// Self-balancing AVL tree of i32 keys.
//
// Each node stores its subtree height; after every insertion or deletion we
// walk back up the recursion stack rebalancing nodes whose balance factor
// (left_height - right_height) exceeds |1|. Worst-case height stays
// <= 1.44 * log2(N+1), so all ops are O(log N).
//
// No custom Drop needed: tree height is logarithmic, so recursive drop
// can't blow the stack the way a linked list does.

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

// Take ownership, restructure, return new root of subtree.

fn rotate_right(mut root: Box<Node>) -> Box<Node> {
    let mut new_root = root.left.take().expect("rotate_right needs left child");
    root.left = new_root.right.take();
    root.update_height();
    new_root.right = Some(root);
    new_root.update_height();
    new_root
}

fn rotate_left(mut root: Box<Node>) -> Box<Node> {
    let mut new_root = root.right.take().expect("rotate_left needs right child");
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
        // Left-heavy. If the left child is right-heavy, do a left-right
        // double rotation by first rotating the child.
        if node.left.as_ref().unwrap().balance_factor() < 0 {
            node.left = Some(rotate_left(node.left.take().unwrap()));
        }
        rotate_right(node)
    } else if bf < -1 {
        // Right-heavy; mirror image.
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
                return Some(n); // duplicate: no-op
            }
            Some(rebalance(n))
        }
    }
}

// Remove and return the smallest value in this subtree, plus the
// (possibly rebalanced) remainder. Used by delete when the target has
// two children: we promote its in-order successor.
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
                let (new_right, succ_value) = take_min(r);
                n.value = succ_value;
                n.left = Some(l);
                n.right = new_right;
                Some(rebalance(n))
            }
        };
        (replacement, true)
    }
}

impl AvlTree {
    pub fn new() -> Self {
        AvlTree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        self.root = insert_node(self.root.take(), value);
    }

    pub fn find(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.value {
                return true;
            }
            current = if value < node.value { &node.left } else { &node.right };
        }
        false
    }

    pub fn delete(&mut self, value: i32) -> bool {
        let (new_root, found) = delete_node(self.root.take(), value);
        self.root = new_root;
        found
    }

    pub fn height(&self) -> i32 {
        height(&self.root)
    }

    fn write_inorder(
        node: &Option<Box<Node>>,
        f: &mut std::fmt::Formatter<'_>,
        first: &mut bool,
    ) -> std::fmt::Result {
        if let Some(n) = node {
            Self::write_inorder(&n.left, f, first)?;
            if !*first {
                write!(f, ", ")?;
            }
            write!(f, "{}", n.value)?;
            *first = false;
            Self::write_inorder(&n.right, f, first)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for AvlTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        Self::write_inorder(&self.root, f, &mut first)?;
        write!(f, "]")
    }
}

fn main() {
    let mut tree = AvlTree::new();

    println!("Inserting 1..=7 in increasing order");
    println!("(a plain BST would degenerate into a right spine of height 7):");
    for i in 1..=7 {
        tree.insert(i);
        println!("  insert({i})  height={}  inorder={}", tree.height(), tree);
    }

    println!();
    println!("find(4):       {}", tree.find(4));
    println!("find(99):      {}", tree.find(99));

    println!();
    println!("Deletions:");
    for v in [4, 1, 7, 99] {
        let ok = tree.delete(v);
        println!(
            "  delete({v:2})  removed={:<5}  height={}  inorder={}",
            ok, tree.height(), tree
        );
    }
}

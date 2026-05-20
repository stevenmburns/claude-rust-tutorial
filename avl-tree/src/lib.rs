//! Self-balancing AVL tree of `i32` keys.
//!
//! Mirror of the tutorial's `avl_tree.rs`, packaged as a library crate.
//! All mutating helpers use the "take ownership, return ownership" pattern
//! over `Box<Node>` to satisfy the borrow checker without `unsafe`.
//!
//! ## Example
//!
//! ```
//! use avl_tree::AvlTree;
//!
//! let mut t = AvlTree::new();
//! for v in [3, 1, 4, 1, 5, 9, 2, 6] {
//!     t.insert(v);
//! }
//! assert_eq!(t.len(), 7); // the duplicate 1 was a no-op
//! assert!(t.find(4));
//! assert!(!t.find(7));
//! assert_eq!(t.to_vec(), vec![1, 2, 3, 4, 5, 6, 9]);
//! ```

use std::fmt;

struct Node {
    value: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/// A self-balancing AVL tree of `i32` keys.
///
/// Acts as a set: each value may appear at most once. Operations are
/// `O(log n)` worst case because the tree's height is bounded by
/// `1.44 * log2(n + 1)`.
pub struct AvlTree {
    root: Option<Box<Node>>,
    len: usize,
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

fn insert_node(node: Option<Box<Node>>, value: i32) -> (Option<Box<Node>>, bool) {
    match node {
        None => (Some(Node::leaf(value)), true),
        Some(mut n) => {
            let inserted = if value < n.value {
                let (new_left, ins) = insert_node(n.left.take(), value);
                n.left = new_left;
                ins
            } else if value > n.value {
                let (new_right, ins) = insert_node(n.right.take(), value);
                n.right = new_right;
                ins
            } else {
                false // duplicate: no-op
            };
            if inserted {
                (Some(rebalance(n)), true)
            } else {
                (Some(n), false)
            }
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

fn collect_inorder(node: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        collect_inorder(&n.left, out);
        out.push(n.value);
        collect_inorder(&n.right, out);
    }
}

impl AvlTree {
    /// Create an empty tree.
    pub fn new() -> Self {
        AvlTree { root: None, len: 0 }
    }

    /// Number of distinct values currently in the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// `true` if the tree contains no values.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Height of the tree. An empty tree has height 0; a single-node tree
    /// has height 1. Bounded above by `1.44 * log2(n + 1)`.
    pub fn height(&self) -> i32 {
        height(&self.root)
    }

    /// Insert `value`. Returns `true` if the value was newly inserted,
    /// `false` if it was already present.
    pub fn insert(&mut self, value: i32) -> bool {
        let (new_root, inserted) = insert_node(self.root.take(), value);
        self.root = new_root;
        if inserted {
            self.len += 1;
        }
        inserted
    }

    /// `true` if the value is present in the tree.
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

    /// Remove `value`. Returns `true` if the value was present and removed,
    /// `false` if it was not in the tree.
    pub fn delete(&mut self, value: i32) -> bool {
        let (new_root, removed) = delete_node(self.root.take(), value);
        self.root = new_root;
        if removed {
            self.len -= 1;
        }
        removed
    }

    /// Collect all values into a sorted `Vec` via in-order traversal.
    pub fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::with_capacity(self.len);
        collect_inorder(&self.root, &mut out);
        out
    }
}

impl Default for AvlTree {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AvlTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        write_inorder(&self.root, f, &mut first)?;
        write!(f, "]")
    }
}

fn write_inorder(
    node: &Option<Box<Node>>,
    f: &mut fmt::Formatter<'_>,
    first: &mut bool,
) -> fmt::Result {
    if let Some(n) = node {
        write_inorder(&n.left, f, first)?;
        if !*first {
            write!(f, ", ")?;
        }
        write!(f, "{}", n.value)?;
        *first = false;
        write_inorder(&n.right, f, first)?;
    }
    Ok(())
}

#[cfg(test)]
impl AvlTree {
    /// Verify all AVL invariants and return the tree's height.
    /// Panics if any invariant is violated. Test-only.
    fn check_invariants(&self) {
        check_node(&self.root, None, None);
    }
}

#[cfg(test)]
fn check_node(
    node: &Option<Box<Node>>,
    lower: Option<i32>,
    upper: Option<i32>,
) -> i32 {
    let Some(n) = node else { return 0; };
    if let Some(lo) = lower {
        assert!(n.value > lo, "BST invariant: {} not > lower bound {lo}", n.value);
    }
    if let Some(hi) = upper {
        assert!(n.value < hi, "BST invariant: {} not < upper bound {hi}", n.value);
    }
    let lh = check_node(&n.left, lower, Some(n.value));
    let rh = check_node(&n.right, Some(n.value), upper);
    assert!(
        (lh - rh).abs() <= 1,
        "balance factor out of range at value {}: lh={lh} rh={rh}",
        n.value
    );
    let expected = 1 + lh.max(rh);
    assert_eq!(
        n.height, expected,
        "stored height {} != computed height {} at value {}",
        n.height, expected, n.value
    );
    n.height
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    // Deterministic LCG so tests are reproducible without an external RNG dep.
    struct Lcg(u64);
    impl Lcg {
        fn new(seed: u64) -> Self { Lcg(seed) }
        fn next(&mut self) -> u64 {
            self.0 = self.0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0
        }
    }

    #[test]
    fn new_is_empty() {
        let t = AvlTree::new();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
        assert_eq!(t.height(), 0);
        assert!(!t.find(0));
        assert_eq!(t.to_vec(), Vec::<i32>::new());
        t.check_invariants();
    }

    #[test]
    fn single_insert_find_delete() {
        let mut t = AvlTree::new();
        assert!(t.insert(42));
        assert_eq!(t.len(), 1);
        assert_eq!(t.height(), 1);
        assert!(t.find(42));
        assert!(!t.find(41));
        t.check_invariants();

        assert!(t.delete(42));
        assert!(t.is_empty());
        assert_eq!(t.height(), 0);
        t.check_invariants();
    }

    #[test]
    fn duplicate_insert_is_noop() {
        let mut t = AvlTree::new();
        assert!(t.insert(7));
        assert!(!t.insert(7)); // second insert reports "already present"
        assert_eq!(t.len(), 1);
        t.check_invariants();
    }

    #[test]
    fn delete_missing_returns_false() {
        let mut t = AvlTree::new();
        assert!(!t.delete(1)); // empty tree
        t.insert(5);
        t.insert(10);
        assert!(!t.delete(7)); // present-keys-only
        assert_eq!(t.len(), 2);
        t.check_invariants();
    }

    #[test]
    fn monotonic_insert_stays_logarithmic() {
        // Worst case for a naive BST: ascending input would build a right
        // spine of height N. AVL must keep height <= 1.44 * log2(N+1).
        let mut t = AvlTree::new();
        let n = 1023;
        for v in 0..n {
            t.insert(v);
        }
        t.check_invariants();
        assert_eq!(t.len() as i32, n);
        // log2(1024) = 10, so a hard upper bound of 14 (1.44 * 10) is loose.
        assert!(t.height() <= 14, "height {} too large for n={n}", t.height());
    }

    #[test]
    fn descending_insert_stays_logarithmic() {
        let mut t = AvlTree::new();
        for v in (0..1023).rev() {
            t.insert(v);
        }
        t.check_invariants();
        assert!(t.height() <= 14);
    }

    #[test]
    fn delete_root_with_two_children() {
        // Build a small tree where the root has two children, then delete
        // the root. Exercises the in-order-successor (take_min) branch.
        let mut t = AvlTree::new();
        for v in [10, 5, 15, 3, 7, 12, 20] {
            t.insert(v);
        }
        assert!(t.find(10));
        assert!(t.delete(10));
        assert!(!t.find(10));
        assert_eq!(t.len(), 6);
        // Surviving values must still be sorted in-order.
        assert_eq!(t.to_vec(), vec![3, 5, 7, 12, 15, 20]);
        t.check_invariants();
    }

    #[test]
    fn inorder_is_sorted() {
        let mut t = AvlTree::new();
        for v in [50, 30, 70, 20, 40, 60, 80, 10] {
            t.insert(v);
        }
        let sorted = t.to_vec();
        let mut expected = sorted.clone();
        expected.sort_unstable();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn display_matches_inorder_vec() {
        let mut t = AvlTree::new();
        for v in [3, 1, 2] {
            t.insert(v);
        }
        assert_eq!(format!("{t}"), "[1, 2, 3]");
    }

    #[test]
    fn oracle_random() {
        // 5000 mixed operations on a small key universe (forces frequent
        // collisions/duplicates). Each op is compared against BTreeSet
        // and AVL invariants are checked after every step.
        let mut tree = AvlTree::new();
        let mut oracle: BTreeSet<i32> = BTreeSet::new();
        let mut rng = Lcg::new(0xc0ffee_u64);

        for step in 0..5000 {
            let r = rng.next();
            let value = ((r >> 32) as i32).rem_euclid(200);
            let op = (r >> 28) & 3;

            match op {
                0 | 1 => {
                    let got = tree.insert(value);
                    let want = oracle.insert(value);
                    assert_eq!(got, want, "insert disagree step={step} val={value}");
                }
                2 => {
                    let got = tree.delete(value);
                    let want = oracle.remove(&value);
                    assert_eq!(got, want, "delete disagree step={step} val={value}");
                }
                _ => {
                    let got = tree.find(value);
                    let want = oracle.contains(&value);
                    assert_eq!(got, want, "find disagree step={step} val={value}");
                }
            }

            assert_eq!(tree.len(), oracle.len(), "len disagree at step {step}");
            tree.check_invariants();
        }

        let from_tree = tree.to_vec();
        let from_oracle: Vec<i32> = oracle.iter().copied().collect();
        assert_eq!(from_tree, from_oracle, "final in-order traversal mismatch");
    }
}

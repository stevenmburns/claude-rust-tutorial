// Benchmark: 1M sequential inserts + 1M sequential deletes on AVL tree,
// using RAW POINTERS (*mut Node) inside `unsafe` blocks.
//
// This mirrors bench_avl_raw.cpp: no Option/Box wrapper, no auto-cleanup,
// manual Box::from_raw for deallocation. Allocator is the same global
// allocator that Box uses, so the comparison vs safe Rust isolates the
// language-level cost (write-None-via-.take(), Option unwrapping) from
// the allocator. The comparison vs raw-ptr C++ isolates codegen.

use std::ptr;
use std::time::Instant;

struct Node {
    value: i32,
    height: i32,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    fn new_leaf(value: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            value,
            height: 1,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }))
    }
}

unsafe fn height(n: *const Node) -> i32 {
    if n.is_null() { 0 } else { (*n).height }
}

unsafe fn update_height(n: *mut Node) {
    (*n).height = 1 + height((*n).left).max(height((*n).right));
}

unsafe fn balance_factor(n: *const Node) -> i32 {
    height((*n).left) - height((*n).right)
}

unsafe fn rotate_right(root: *mut Node) -> *mut Node {
    let new_root = (*root).left;
    (*root).left = (*new_root).right;
    update_height(root);
    (*new_root).right = root;
    update_height(new_root);
    new_root
}

unsafe fn rotate_left(root: *mut Node) -> *mut Node {
    let new_root = (*root).right;
    (*root).right = (*new_root).left;
    update_height(root);
    (*new_root).left = root;
    update_height(new_root);
    new_root
}

unsafe fn rebalance(node: *mut Node) -> *mut Node {
    update_height(node);
    let bf = balance_factor(node);
    if bf > 1 {
        if balance_factor((*node).left) < 0 {
            (*node).left = rotate_left((*node).left);
        }
        rotate_right(node)
    } else if bf < -1 {
        if balance_factor((*node).right) > 0 {
            (*node).right = rotate_right((*node).right);
        }
        rotate_left(node)
    } else {
        node
    }
}

unsafe fn insert_node(node: *mut Node, value: i32) -> *mut Node {
    if node.is_null() {
        return Node::new_leaf(value);
    }
    if value < (*node).value {
        (*node).left = insert_node((*node).left, value);
    } else if value > (*node).value {
        (*node).right = insert_node((*node).right, value);
    } else {
        return node;
    }
    rebalance(node)
}

unsafe fn take_min(node: *mut Node) -> (*mut Node, i32) {
    if !(*node).left.is_null() {
        let (new_left, min_value) = take_min((*node).left);
        (*node).left = new_left;
        (rebalance(node), min_value)
    } else {
        let v = (*node).value;
        let right = (*node).right;
        drop(Box::from_raw(node));
        (right, v)
    }
}

unsafe fn delete_node(node: *mut Node, value: i32) -> (*mut Node, bool) {
    if node.is_null() {
        return (ptr::null_mut(), false);
    }
    if value < (*node).value {
        let (new_left, found) = delete_node((*node).left, value);
        (*node).left = new_left;
        return (rebalance(node), found);
    }
    if value > (*node).value {
        let (new_right, found) = delete_node((*node).right, value);
        (*node).right = new_right;
        return (rebalance(node), found);
    }
    // Found.
    if (*node).left.is_null() && (*node).right.is_null() {
        drop(Box::from_raw(node));
        return (ptr::null_mut(), true);
    }
    if (*node).left.is_null() {
        let r = (*node).right;
        drop(Box::from_raw(node));
        return (r, true);
    }
    if (*node).right.is_null() {
        let l = (*node).left;
        drop(Box::from_raw(node));
        return (l, true);
    }
    let (new_right, succ) = take_min((*node).right);
    (*node).value = succ;
    (*node).right = new_right;
    (rebalance(node), true)
}

unsafe fn free_all(n: *mut Node) {
    if n.is_null() {
        return;
    }
    free_all((*n).left);
    free_all((*n).right);
    drop(Box::from_raw(n));
}

pub struct AvlTree {
    root: *mut Node,
}

impl AvlTree {
    pub fn new() -> Self {
        AvlTree { root: ptr::null_mut() }
    }
    pub fn insert(&mut self, value: i32) {
        unsafe {
            self.root = insert_node(self.root, value);
        }
    }
    pub fn delete(&mut self, value: i32) -> bool {
        unsafe {
            let (new_root, found) = delete_node(self.root, value);
            self.root = new_root;
            found
        }
    }
    pub fn height(&self) -> i32 {
        unsafe { height(self.root) }
    }
}

impl Drop for AvlTree {
    fn drop(&mut self) {
        unsafe { free_all(self.root); }
    }
}

fn main() {
    const N: i32 = 1_000_000;
    println!("Rust unsafe (raw *mut Node)  N={N} sequential inserts + N sequential deletes");
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

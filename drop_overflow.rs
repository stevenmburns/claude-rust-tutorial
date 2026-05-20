struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert(&mut self, value: i32) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }
}

fn main() {
    let n: i32 = 1_000_000;
    let mut list = LinkedList::new();
    for i in 0..n {
        list.insert(i);
    }
    println!("built list of {n} nodes; dropping now...");
    drop(list);
    println!("dropped cleanly");
}

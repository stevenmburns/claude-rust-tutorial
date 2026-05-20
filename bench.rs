use std::hint::black_box;
use std::time::Instant;

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

    pub fn find(&self, value: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == value {
                return true;
            }
            current = &node.next;
        }
        false
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

fn main() {
    const N: i32 = 1_000_000;
    const M: i32 = 50;

    println!("Rust N={N} full-traversal misses M={M}");
    for trial in 1..=3 {
        let mut list = Box::new(LinkedList::new());

        let t0 = Instant::now();
        for i in 0..N {
            list.insert(i);
        }
        let t1 = Instant::now();

        let mut hits: u32 = 0;
        for _ in 0..M {
            if list.find(-1) {
                hits += 1;
            }
        }
        black_box(hits);
        let t2 = Instant::now();

        drop(list);
        let t3 = Instant::now();

        let build = t1.duration_since(t0).as_secs_f64() * 1000.0;
        let find = t2.duration_since(t1).as_secs_f64() * 1000.0;
        let dropt = t3.duration_since(t2).as_secs_f64() * 1000.0;
        println!("  trial {trial}  build={build:7.1} ms  find={find:7.1} ms  drop={dropt:7.1} ms");
    }
}

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

    pub fn delete(&mut self, value: i32) -> bool {
        let mut current = &mut self.head;
        loop {
            match current {
                None => return false,
                Some(node) if node.value == value => {
                    *current = node.next.take();
                    return true;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
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

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut current = &self.head;
        let mut first = true;
        while let Some(node) = current {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", node.value)?;
            first = false;
            current = &node.next;
        }
        write!(f, "]")
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert(3);
    list.insert(2);
    list.insert(1);
    println!("after inserts:  {}", list);

    println!("find(2):        {}", list.find(2));
    println!("find(9):        {}", list.find(9));

    println!("delete(2):      {}", list.delete(2));
    println!("after delete:   {}", list);

    println!("delete(1):      {}", list.delete(1));
    println!("after delete:   {}", list);

    println!("delete(9):      {}", list.delete(9));
    println!("after delete:   {}", list);
}

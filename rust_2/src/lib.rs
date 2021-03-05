mod node;
use node::Node;

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    pub fn display(&self) {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        println!("{}", result);
    }
}

#[test]
fn basic_test() {
    let mut list = LinkedList::new();
    assert_eq!(list.get_size(), 0);
    assert!(list.is_empty());

    list.push(15);
    assert_eq!(list.get_size(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop(), Some(15));
    assert!(list.is_empty());
}

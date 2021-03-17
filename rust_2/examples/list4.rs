impl Node {
    fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node { value, next }
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            size: 0,
        }
    }
}

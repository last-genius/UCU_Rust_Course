pub struct Node {
    pub value: u32,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
        // And so on...

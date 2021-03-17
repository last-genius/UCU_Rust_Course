pub fn push(&mut self, value: u32) {
    let new_node = Box::new(Node::new(value, self.head));
    self.head = Some(new_node);
    self.size += 1;
}

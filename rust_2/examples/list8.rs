pub fn pop(&mut self) -> Option<u32> {
    let node = self.head.take()?;
    self.head = node.next;
    self.size -= 1;
    Some(node.value)
}

pub fn display(&self) {
    let mut current: &Option<Box<Node>> = &self.head;
    let mut result = String::new();
    loop {
        match current {
            Some(node) => {
                result = format!("{} {}", result, node.value);
                current = &node.next;
            },
            None => break,
        }
    }
    println!("{}", result);
}

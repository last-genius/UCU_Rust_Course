struct Point<T> {
    x: T,
    y: T,
}

// A generic impl block also has special syntax
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

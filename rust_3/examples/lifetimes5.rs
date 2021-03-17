fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

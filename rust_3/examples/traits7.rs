fn main() {
    let s: i32 = 64;
    let u = s;
    println!("{}", s);
}

// Does not implement Copy, will move out
fn main() {
    let s: String = "text".to_string();
    let u = s;
    println!("{}", s);
}

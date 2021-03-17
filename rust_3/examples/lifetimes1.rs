fn main() {
    let a;

    {
        let b = 5;
        a = &b;
    }

    println!("a: {}", a);
}

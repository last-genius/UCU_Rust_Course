fn adder<T: std::ops::Add<Output = T>>(a: T, b: T) -> String {
    format!("{} + {} = {}", a, b, a + b)
}

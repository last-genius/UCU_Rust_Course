fn adder<T>(a: T, b: T) -> String
where
    T: std::ops::Add<Output = T> + std::fmt::Display + Copy,
{
    format!("{} + {} = {}", a, b, a + b)
}

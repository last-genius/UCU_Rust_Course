fn function<T, U>(t: &T, u: &U) -> String
where
    T: Report + Clone,
    U: Display + Clone,
{
}

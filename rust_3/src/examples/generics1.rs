// Option is generic over type T
pub enum Option<T> {
    None,
    Some(T),
}

// Result is generic over types T, E
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

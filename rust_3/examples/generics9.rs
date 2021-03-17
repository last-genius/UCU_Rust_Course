error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> src/main.rs:2:29
  |
2 |     format!("{} + {} = {}", a, b, a + b)
  |                             ^ `T` cannot be formatted with the default formatter
  |
  = note: required by `std::fmt::Display::fmt`
help: consider further restricting this bound
  |
1 | fn adder<T: std::ops::Add<Output = T> + std::fmt::Display>(a: T, b: T) -> String {
  |                                       ^^^^^^^^^^^^^^^^^^^

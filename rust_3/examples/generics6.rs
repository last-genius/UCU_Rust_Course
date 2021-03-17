error[E0369]: cannot add `T` to `T`
 --> src/main.rs:2:37
  |
2 |     format!("{} + {} = {}", a, b, a + b)
  |                                   - ^ - T
  |                                   |
  |                                   T
  |
help: consider restricting type parameter `T`
  |
1 | fn adder<T: std::ops::Add<Output = T>>(a: T, b: T) -> String {
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^

let a = 5;
let b = 32;
let c = |x: i32| x + a;

let d = &mut a; // Error - can't borrow as mutable when
                      // borrowed as immutable
println!("{}", b); // Works fine - closure did not capture it

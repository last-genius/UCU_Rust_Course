let val: u8 = 42;
let boxed: Box<u8> = Box::new(val);

// We can easily dereference the box
println!("{}", boxed); // prints 42

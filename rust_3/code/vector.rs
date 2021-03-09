// Standard initialization
let v: Vec<i32> = Vec::new();

// Macro initialization
let mut v = vec![1, 1, 1, 1, 1];
let mut v = vec![1; 5];

v.push(4);
let last_element = v.pop();

// Slicing
&a[1..4]

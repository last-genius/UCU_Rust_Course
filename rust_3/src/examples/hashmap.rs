// HashMap is not included in the prelude, you
// have to 'use' it. Might change in Rust 2021
use std::collections::HashMap;

// This will get inferred by the compiler as HashMap<String, i32>
let mut grades = HashMap::new();

grades.insert(String::from("Student1"), 100);
grades.insert(String::from("Student2"), 90);

// Will panic if the key is absent
grades["Student1"];

// Will not panic
grades.get("Student1");

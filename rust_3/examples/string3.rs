// Rust's Strings implement Deref to &str, so it's 
// better to take string slices as function parameters
fn takes_str(s: &str) { }

let s = String::from("Hello");

// Will deref from &String to &str
// This is known as Deref coercion
takes_str(&s);

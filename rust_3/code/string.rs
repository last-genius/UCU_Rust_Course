// This dynamic string is allocated on the heap
let mut s = String::new();

// This string is compiled into the binary
let data = "utf-8 string";

let s = data.to_string();
let s = String::from("utf-8 string");

// String is a mutable growable type
s.push('a');
s.push_str("text");
s.push_str(data);

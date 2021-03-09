// Might cause Rust to panic if we access beyond boundaries
&v[0];

// Will never panic but is pretty ugly
match v.get(2) {
    Some(elm) => println!("{}", elm),
    None => println!("There is no such element"),
}

for i in &v {
    println!("{}", i);
}

for i in &mut v {
    *i += 50;
}
